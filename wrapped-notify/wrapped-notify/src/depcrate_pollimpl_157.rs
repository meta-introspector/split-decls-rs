// Generated macro for impl_157 (impl)
macro_rules! Depcrate_pollimpl_157 {
() => {
// Module: crate::poll
// Provides: {"impl_157"}
// Dependencies: {}
impl Watcher for PollWatcher { # [doc = " Create a new [`PollWatcher`]."] fn new < F : EventHandler > (event_handler : F , config : Config) -> crate :: Result < Self > { Self :: new (event_handler , config) } fn watch (& mut self , path : & Path , recursive_mode : RecursiveMode) -> crate :: Result < () > { self . watch_inner (path , recursive_mode) ; Ok (()) } fn unwatch (& mut self , path : & Path) -> crate :: Result < () > { self . unwatch_inner (path) } fn kind () -> crate :: WatcherKind { crate :: WatcherKind :: PollWatcher } }
};
}
