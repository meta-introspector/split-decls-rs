// Generated macro for run (function)
macro_rules! Depcrate_buildrun {
() => {
// Module: crate::build
// Provides: {"run"}
// Dependencies: {}
# [doc = " Executes the build process."] pub fn run () -> Result < () , String > { let mut args = match BuildArg :: new () ? { Some (args) => args , None => return Ok (()) , } ; args . config_info . setup_gcc_path () ? ; build_codegen (& mut args) ? ; Ok (()) }
};
}
