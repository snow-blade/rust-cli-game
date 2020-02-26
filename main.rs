#[derive(Debug)]
struct Fighter{
name:String,
damage: u32,
class:String,
resistance:String,
health: u32,
}

impl Fighter{
    fn attack(&self,other:&mut Fighter){
        if other.health>self.damage{
        other.health-=self.damage;
        }
        else
        {
        
        other.health=0;
            
        }
    }
    
}
fn main() {
let mut alex=Fighter{
name: String::from("Alex"),
damage: 21,
class: String::from("firewatch"),
resistance: String::from("icenight"),
health: 100,
};
let franck=Fighter{
    name: String::from("franck"),
damage: 12,
class: String::from("Greencaster"),
resistance: String::from("earthling"),
health: 100,
};
println!("{}",alex.damage);

while(alex.health!=0){
    franck.attack(&mut alex);
    println!("ouch...alex was attacked his health now is: {}",alex.health);
}



    if alex.health==0{
    println!("Alex is dead");
}}
