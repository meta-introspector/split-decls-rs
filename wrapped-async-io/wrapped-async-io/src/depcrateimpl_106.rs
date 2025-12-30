// Generated macro for impl_106 (impl)
macro_rules! Depcrateimpl_106 {
() => {
// Module: crate
// Provides: {"impl_106"}
// Dependencies: {}
impl < T > Drop for Async < T > { fn drop (& mut self) { if self . io . is_some () { Reactor :: get () . remove_io (& self . source) . ok () ; self . io . take () ; } } }
};
}
