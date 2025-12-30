// Generated macro for run_rustc (function)
macro_rules! Depcrate_rust_toolsrun_rustc {
() => {
// Module: crate::rust_tools
// Provides: {"run_rustc"}
// Dependencies: {}
pub fn run_rustc () -> Result < () , String > { let Some (tools) = RustcTools :: new ("rustc") ? else { return Ok (()) } ; let mut command = tools . config . rustc_command_vec () ; for arg in & tools . args { command . push (arg) ; } exec (& command , & tools . env) }
};
}
