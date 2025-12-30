// Generated macro for impl_44 (impl)
macro_rules! Depcrate_fseventimpl_44 {
() => {
// Module: crate::fsevent
// Provides: {"impl_44"}
// Dependencies: {}
impl Watcher for FsEventWatcher { # [doc = " Create a new watcher."] fn new < F : EventHandler > (event_handler : F , _config : Config) -> Result < Self > { Self :: from_event_handler (Arc :: new (Mutex :: new (event_handler))) } fn watch (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { self . watch_inner (path , recursive_mode) } fn paths_mut < 'me > (& 'me mut self) -> Box < dyn PathsMut + 'me > { Box :: new (FsEventPathsMut :: new (self)) } fn unwatch (& mut self , path : & Path) -> Result < () > { self . unwatch_inner (path) } fn configure (& mut self , config : Config) -> Result < bool > { let (tx , rx) = unbounded () ; self . configure_raw_mode (config , tx) ; rx . recv () ? } fn kind () -> crate :: WatcherKind { crate :: WatcherKind :: Fsevent } }
};
}
