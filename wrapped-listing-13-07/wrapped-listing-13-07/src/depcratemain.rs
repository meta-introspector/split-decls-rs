// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut list = [Rectangle { width : 10 , height : 1 } , Rectangle { width : 3 , height : 5 } , Rectangle { width : 7 , height : 12 } ,] ; list . sort_by_key (| r | r . width) ; println ! ("{list:#?}") ; }
};
}
