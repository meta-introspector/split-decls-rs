// Generated macro for impl_37 (impl)
macro_rules! Depcrate_connectionimpl_37 {
() => {
// Module: crate::connection
// Provides: {"impl_37"}
// Dependencies: {}
impl Clone for ConnectionRef { fn clone (& self) -> Self { self . state . lock ("clone") . ref_count += 1 ; Self (self . 0 . clone ()) } }
};
}
