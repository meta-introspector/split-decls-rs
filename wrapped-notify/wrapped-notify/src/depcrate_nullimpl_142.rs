// Generated macro for impl_142 (impl)
macro_rules! Depcrate_nullimpl_142 {
() => {
// Module: crate::null
// Provides: {"impl_142"}
// Dependencies: {}
impl Watcher for NullWatcher { fn watch (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { Ok (()) } fn unwatch (& mut self , path : & Path) -> Result < () > { Ok (()) } fn new < F : crate :: EventHandler > (event_handler : F , config : Config) -> Result < Self > where Self : Sized , { Ok (NullWatcher) } fn configure (& mut self , config : Config) -> Result < bool > { Ok (false) } fn kind () -> crate :: WatcherKind { crate :: WatcherKind :: NullWatcher } }
};
}
