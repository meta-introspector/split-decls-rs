// Generated macro for exec (function)
macro_rules! Depcrate_rust_toolsexec {
() => {
// Module: crate::rust_tools
// Provides: {"exec"}
// Dependencies: {}
fn exec (input : & [& dyn AsRef < OsStr >] , env : & HashMap < String , String >) -> Result < () , String > { # [cfg (unix)] { let error = crate :: utils :: get_command_inner (input , None , Some (env)) . exec () ; eprintln ! ("execvp syscall failed: {error:?}") ; std :: process :: exit (1) ; } # [cfg (not (unix))] { if crate :: utils :: run_command_with_output_and_env_no_err (input , None , Some (env)) . is_err () { std :: process :: exit (1) ; } Ok (()) } }
};
}
