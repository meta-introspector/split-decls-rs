// Generated macro for impl_96 (impl)
macro_rules! Depcrate_kqueueimpl_96 {
() => {
// Module: crate::kqueue
// Provides: {"impl_96"}
// Dependencies: {}
impl Watcher for KqueueWatcher { # [doc = " Create a new watcher."] fn new < F : EventHandler > (event_handler : F , config : Config) -> Result < Self > { Self :: from_event_handler (Box :: new (event_handler) , config . follow_symlinks ()) } fn watch (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { self . watch_inner (path , recursive_mode) } fn unwatch (& mut self , path : & Path) -> Result < () > { self . unwatch_inner (path) } fn kind () -> crate :: WatcherKind { crate :: WatcherKind :: Kqueue } }
};
}
