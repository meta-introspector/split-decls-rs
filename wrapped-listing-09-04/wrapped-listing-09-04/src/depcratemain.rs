// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let greeting_file_result = File :: open ("hello.txt") ; let greeting_file = match greeting_file_result { Ok (file) => file , Err (error) => panic ! ("Problem opening the file: {error:?}") , } ; }
};
}
