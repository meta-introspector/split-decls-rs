// Generated macro for run_command_with_output_and_env (function)
macro_rules! Depcrate_utilsrun_command_with_output_and_env {
() => {
// Module: crate::utils
// Provides: {"run_command_with_output_and_env"}
// Dependencies: {}
pub fn run_command_with_output_and_env (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : Option < & HashMap < String , String > > ,) -> Result < () , String > { let exit_status = exec_command (input , cwd , env) ? ; check_exit_status (input , cwd , exit_status , None , true) ? ; Ok (()) }
};
}
