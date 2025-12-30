// Generated macro for SynchronizedUpdate (trait)
macro_rules! Depcrate_commandSynchronizedUpdate {
() => {
// Module: crate::command
// Provides: {"SynchronizedUpdate"}
// Dependencies: {}
# [doc = " An interface for types that support synchronized updates."] pub trait SynchronizedUpdate { # [doc = " Performs a set of actions against the given type."] fn sync_update < T > (& mut self , operations : impl FnOnce (& mut Self) -> T) -> io :: Result < T > ; }
};
}
