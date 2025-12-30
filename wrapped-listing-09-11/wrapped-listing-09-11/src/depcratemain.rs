// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { assert_eq ! (last_char_of_first_line ("Hello, world\nHow are you today?") , Some ('d')) ; assert_eq ! (last_char_of_first_line ("") , None) ; assert_eq ! (last_char_of_first_line ("\nhi") , None) ; }
};
}
