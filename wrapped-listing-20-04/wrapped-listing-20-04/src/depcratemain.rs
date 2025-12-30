// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut v = vec ! [1 , 2 , 3 , 4 , 5 , 6] ; let r = & mut v [..] ; let (a , b) = r . split_at_mut (3) ; assert_eq ! (a , & mut [1 , 2 , 3]) ; assert_eq ! (b , & mut [4 , 5 , 6]) ; }
};
}
