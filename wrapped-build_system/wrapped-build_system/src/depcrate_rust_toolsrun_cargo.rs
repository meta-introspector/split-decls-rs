// Generated macro for run_cargo (function)
macro_rules! Depcrate_rust_toolsrun_cargo {
() => {
// Module: crate::rust_tools
// Provides: {"run_cargo"}
// Dependencies: {}
pub fn run_cargo () -> Result < () , String > { let Some (mut tools) = RustcTools :: new ("cargo") ? else { return Ok (()) } ; let rustflags = tools . env . get ("RUSTFLAGS") . cloned () . unwrap_or_default () ; tools . env . insert ("RUSTDOCFLAGS" . to_string () , rustflags) ; let mut command : Vec < & dyn AsRef < OsStr > > = vec ! [& "cargo" , & tools . toolchain] ; for arg in & tools . args { command . push (arg) ; } exec (& command , & tools . env) }
};
}
