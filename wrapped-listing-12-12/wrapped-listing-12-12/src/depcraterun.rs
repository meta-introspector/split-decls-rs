// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (config : Config) -> Result < () , Box < dyn Error > > { let contents = fs :: read_to_string (config . file_path) ? ; println ! ("With text:\n{contents}") ; Ok (()) }
};
}
