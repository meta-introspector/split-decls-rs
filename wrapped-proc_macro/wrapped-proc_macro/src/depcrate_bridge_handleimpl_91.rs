// Generated macro for impl_91 (impl)
macro_rules! Depcrate_bridge_handleimpl_91 {
() => {
// Module: crate::bridge::handle
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > IndexMut < Handle > for OwnedStore < T > { fn index_mut (& mut self , h : Handle) -> & mut T { self . data . get_mut (& h) . expect ("use-after-free in `proc_macro` handle") } }
};
}
