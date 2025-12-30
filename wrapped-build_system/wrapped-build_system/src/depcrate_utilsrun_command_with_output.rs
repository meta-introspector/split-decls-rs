// Generated macro for run_command_with_output (function)
macro_rules! Depcrate_utilsrun_command_with_output {
() => {
// Module: crate::utils
// Provides: {"run_command_with_output"}
// Dependencies: {}
pub fn run_command_with_output (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > ,) -> Result < () , String > { let exit_status = exec_command (input , cwd , None) ? ; check_exit_status (input , cwd , exit_status , None , true) ? ; Ok (()) }
};
}
