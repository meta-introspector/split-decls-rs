// Generated macro for impl_70 (impl)
macro_rules! Depcrate_processimpl_70 {
() => {
// Module: crate::process
// Provides: {"impl_70"}
// Dependencies: {}
impl Drop for CommandReader { fn drop (& mut self) { if let Err (error) = self . close () { log :: warn ! ("{}" , error) ; } } }
};
}
