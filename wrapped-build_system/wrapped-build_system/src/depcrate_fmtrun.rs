// Generated macro for run (function)
macro_rules! Depcrate_fmtrun {
() => {
// Module: crate::fmt
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Result < () , String > { let mut check = false ; let args = std :: env :: args () . skip (2) ; for arg in args { match arg . as_str () { "--help" => { show_usage () ; return Ok (()) ; } "--check" => check = true , _ => return Err (format ! ("Unknown option {arg}")) , } } let cmd : & [& dyn AsRef < OsStr >] = if check { & [& "cargo" , & "fmt" , & "--check"] } else { & [& "cargo" , & "fmt"] } ; run_command_with_output (cmd , Some (Path :: new ("."))) ? ; run_command_with_output (cmd , Some (Path :: new ("build_system"))) ? ; run_rustfmt_recursively ("tests/run" , check) }
};
}
