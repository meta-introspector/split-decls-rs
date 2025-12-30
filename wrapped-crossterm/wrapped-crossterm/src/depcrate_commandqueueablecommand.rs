// Generated macro for QueueableCommand (trait)
macro_rules! Depcrate_commandQueueableCommand {
() => {
// Module: crate::command
// Provides: {"QueueableCommand"}
// Dependencies: {}
# [doc = " An interface for types that can queue commands for further execution."] pub trait QueueableCommand { # [doc = " Queues the given command for further execution."] fn queue (& mut self , command : impl Command) -> io :: Result < & mut Self > ; }
};
}
