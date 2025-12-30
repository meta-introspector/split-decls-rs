// Generated macro for impl_132 (impl)
macro_rules! Depcrate_windowsimpl_132 {
() => {
// Module: crate::windows
// Provides: {"impl_132"}
// Dependencies: {}
impl Watcher for ReadDirectoryChangesWatcher { fn new < F : EventHandler > (event_handler : F , _config : Config) -> Result < Self > { let (meta_tx , _) = unbounded () ; let event_handler = Arc :: new (Mutex :: new (event_handler)) ; Self :: create (event_handler , meta_tx) } fn watch (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { self . watch_inner (path , recursive_mode) } fn unwatch (& mut self , path : & Path) -> Result < () > { self . unwatch_inner (path) } fn configure (& mut self , config : Config) -> Result < bool > { let (tx , rx) = bounded (1) ; self . tx . send (Action :: Configure (config , tx)) ? ; rx . recv () ? } fn kind () -> crate :: WatcherKind { WatcherKind :: ReadDirectoryChangesWatcher } }
};
}
