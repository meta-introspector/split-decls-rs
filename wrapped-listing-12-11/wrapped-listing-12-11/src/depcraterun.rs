// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (config : Config) { let contents = fs :: read_to_string (config . file_path) . expect ("Should have been able to read the file") ; println ! ("With text:\n{contents}") ; }
};
}
