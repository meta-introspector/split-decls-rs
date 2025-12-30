// Generated macro for impl_88 (impl)
macro_rules! Depcrate_endpointimpl_88 {
() => {
// Module: crate::endpoint
// Provides: {"impl_88"}
// Dependencies: {}
impl Drop for EndpointRef { fn drop (& mut self) { let endpoint = & mut * self . 0 . state . lock () . unwrap () ; if let Some (x) = endpoint . ref_count . checked_sub (1) { endpoint . ref_count = x ; if x == 0 { if let Some (task) = endpoint . driver . take () { task . wake () ; } } } } }
};
}
