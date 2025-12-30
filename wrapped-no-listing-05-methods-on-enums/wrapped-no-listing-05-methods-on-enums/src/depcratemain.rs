// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { enum Message { Quit , Move { x : i32 , y : i32 } , Write (String) , ChangeColor (i32 , i32 , i32) , } impl Message { fn call (& self) { } } let m = Message :: Write (String :: from ("hello")) ; m . call () ; }
};
}
