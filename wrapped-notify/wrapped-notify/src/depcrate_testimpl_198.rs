// Generated macro for impl_198 (impl)
macro_rules! Depcrate_testimpl_198 {
() => {
// Module: crate::test
// Provides: {"impl_198"}
// Dependencies: {}
impl < W : Watcher > TestWatcher < W > { pub fn watch_recursively (& mut self , path : impl AsRef < Path >) { self . watch (path , RecursiveMode :: Recursive) ; } pub fn watch_nonrecursively (& mut self , path : impl AsRef < Path >) { self . watch (path , RecursiveMode :: NonRecursive) ; } pub fn watch (& mut self , path : impl AsRef < Path > , recursive_mode : RecursiveMode) { let path = path . as_ref () ; self . watcher . watch (path , recursive_mode) . unwrap_or_else (| e | panic ! ("Unable to watch {:?}: {e:#?}" , path)) } }
};
}
