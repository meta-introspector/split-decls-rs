// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl Drop for PtyProcess { fn drop (& mut self) { if let Ok (WaitStatus :: StillAlive) = self . status () { self . exit (true) . unwrap () ; } } }
};
}
