// Generated macro for run_command_with_output_and_env_no_err (function)
macro_rules! Depcrate_utilsrun_command_with_output_and_env_no_err {
() => {
// Module: crate::utils
// Provides: {"run_command_with_output_and_env_no_err"}
// Dependencies: {}
# [cfg (not (unix))] pub fn run_command_with_output_and_env_no_err (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : Option < & HashMap < String , String > > ,) -> Result < () , String > { let exit_status = exec_command (input , cwd , env) ? ; check_exit_status (input , cwd , exit_status , None , false) ? ; Ok (()) }
};
}
