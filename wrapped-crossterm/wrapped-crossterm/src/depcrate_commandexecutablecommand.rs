// Generated macro for ExecutableCommand (trait)
macro_rules! Depcrate_commandExecutableCommand {
() => {
// Module: crate::command
// Provides: {"ExecutableCommand"}
// Dependencies: {}
# [doc = " An interface for types that can directly execute commands."] pub trait ExecutableCommand { # [doc = " Executes the given command directly."] fn execute (& mut self , command : impl Command) -> io :: Result < & mut Self > ; }
};
}
