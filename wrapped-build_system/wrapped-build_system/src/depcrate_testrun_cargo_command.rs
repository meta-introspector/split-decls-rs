// Generated macro for run_cargo_command (function)
macro_rules! Depcrate_testrun_cargo_command {
() => {
// Module: crate::test
// Provides: {"run_cargo_command"}
// Dependencies: {}
fn run_cargo_command (command : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : & Env , args : & TestArg ,) -> Result < () , String > { run_cargo_command_with_callback (command , cwd , env , args , | cargo_command , cwd , env | { run_command_with_output_and_env (cargo_command , cwd , Some (env)) ? ; Ok (()) }) }
};
}
