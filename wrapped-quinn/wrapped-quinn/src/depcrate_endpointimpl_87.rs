// Generated macro for impl_87 (impl)
macro_rules! Depcrate_endpointimpl_87 {
() => {
// Module: crate::endpoint
// Provides: {"impl_87"}
// Dependencies: {}
impl Clone for EndpointRef { fn clone (& self) -> Self { self . 0 . state . lock () . unwrap () . ref_count += 1 ; Self (self . 0 . clone ()) } }
};
}
