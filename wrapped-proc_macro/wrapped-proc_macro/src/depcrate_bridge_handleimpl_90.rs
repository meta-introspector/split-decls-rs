// Generated macro for impl_90 (impl)
macro_rules! Depcrate_bridge_handleimpl_90 {
() => {
// Module: crate::bridge::handle
// Provides: {"impl_90"}
// Dependencies: {}
impl < T > Index < Handle > for OwnedStore < T > { type Output = T ; fn index (& self , h : Handle) -> & T { self . data . get (& h) . expect ("use-after-free in `proc_macro` handle") } }
};
}
