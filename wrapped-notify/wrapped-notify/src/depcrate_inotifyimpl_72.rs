// Generated macro for impl_72 (impl)
macro_rules! Depcrate_inotifyimpl_72 {
() => {
// Module: crate::inotify
// Provides: {"impl_72"}
// Dependencies: {}
impl Watcher for INotifyWatcher { # [doc = " Create a new watcher."] fn new < F : EventHandler > (event_handler : F , config : Config) -> Result < Self > { Self :: from_event_handler (Box :: new (event_handler) , config . follow_symlinks ()) } fn watch (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { self . watch_inner (path , recursive_mode) } fn unwatch (& mut self , path : & Path) -> Result < () > { self . unwatch_inner (path) } fn configure (& mut self , config : Config) -> Result < bool > { let (tx , rx) = bounded (1) ; self . channel . send (EventLoopMsg :: Configure (config , tx)) ? ; self . waker . wake () ? ; rx . recv () ? } fn kind () -> crate :: WatcherKind { crate :: WatcherKind :: Inotify } }
};
}
