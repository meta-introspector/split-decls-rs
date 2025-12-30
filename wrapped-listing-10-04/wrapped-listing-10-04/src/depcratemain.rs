// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let number_list = vec ! [34 , 50 , 25 , 100 , 65] ; let result = largest_i32 (& number_list) ; println ! ("The largest number is {result}") ; assert_eq ! (* result , 100) ; let char_list = vec ! ['y' , 'm' , 'a' , 'q'] ; let result = largest_char (& char_list) ; println ! ("The largest char is {result}") ; assert_eq ! (* result , 'y') ; }
};
}
