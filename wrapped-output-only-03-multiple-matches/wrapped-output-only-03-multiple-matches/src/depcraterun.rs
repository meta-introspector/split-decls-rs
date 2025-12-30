// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
pub fn run (config : Config) -> Result < () , Box < dyn Error > > { let contents = fs :: read_to_string (config . file_path) ? ; for line in search (& config . query , & contents) { println ! ("{line}") ; } Ok (()) }
};
}
