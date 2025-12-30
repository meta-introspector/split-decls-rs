// Generated macro for run_command_with_env (function)
macro_rules! Depcrate_utilsrun_command_with_env {
() => {
// Module: crate::utils
// Provides: {"run_command_with_env"}
// Dependencies: {}
pub fn run_command_with_env (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : Option < & HashMap < String , String > > ,) -> Result < Output , String > { let output = get_command_inner (input , cwd , env) . output () . map_err (| e | command_error (input , & cwd , e)) ? ; check_exit_status (input , cwd , output . status , Some (& output) , true) ? ; Ok (output) }
};
}
