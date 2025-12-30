// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args : Vec < String > = env :: args () . collect () ; let config = Config :: build (& args) . unwrap_or_else (| err | { println ! ("Problem parsing arguments: {err}") ; process :: exit (1) ; }) ; println ! ("Searching for {}" , config . query) ; println ! ("In file {}" , config . file_path) ; let contents = fs :: read_to_string (config . file_path) . expect ("Should have been able to read the file") ; println ! ("With text:\n{contents}") ; }
};
}
