// Generated macro for run_command (function)
macro_rules! Depcrate_utilsrun_command {
() => {
// Module: crate::utils
// Provides: {"run_command"}
// Dependencies: {}
pub fn run_command (input : & [& dyn AsRef < OsStr >] , cwd : Option < & Path >) -> Result < Output , String > { run_command_with_env (input , cwd , None) }
};
}
