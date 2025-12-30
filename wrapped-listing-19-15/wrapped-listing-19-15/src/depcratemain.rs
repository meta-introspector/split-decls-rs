// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let msg = Message :: ChangeColor (0 , 160 , 255) ; match msg { Message :: Quit => { println ! ("The Quit variant has no data to destructure.") ; } Message :: Move { x , y } => { println ! ("Move in the x direction {x} and in the y direction {y}") ; } Message :: Write (text) => { println ! ("Text message: {text}") ; } Message :: ChangeColor (r , g , b) => { println ! ("Change color to red {r}, green {g}, and blue {b}") ; } } }
};
}
