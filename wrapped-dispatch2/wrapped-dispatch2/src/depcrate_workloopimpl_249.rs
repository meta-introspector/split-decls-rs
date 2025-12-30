// Generated macro for impl_249 (impl)
macro_rules! Depcrate_workloopimpl_249 {
() => {
// Module: crate::workloop
// Provides: {"impl_249"}
// Dependencies: {}
impl DispatchWorkloop { # [doc = " Create a new [`DispatchWorkloop`]."] pub fn new (label : & str , inactive : bool) -> DispatchRetained < Self > { let label = CString :: new (label) . expect ("Invalid label!") ; unsafe { if inactive { DispatchWorkloop :: __new_inactive (label . as_ptr ()) } else { DispatchWorkloop :: __new (label . as_ptr ()) } } } }
};
}
