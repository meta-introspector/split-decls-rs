// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args : Vec < String > = env :: args () . collect () ; let query = & args [1] ; let file_path = & args [2] ; println ! ("Searching for {query}") ; println ! ("In file {file_path}") ; }
};
}
