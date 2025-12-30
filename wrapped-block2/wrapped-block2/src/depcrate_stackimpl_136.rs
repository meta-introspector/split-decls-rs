// Generated macro for impl_136 (impl)
macro_rules! Depcrate_stackimpl_136 {
() => {
// Module: crate::stack
// Provides: {"impl_136"}
// Dependencies: {}
impl < A , R , Closure : Clone > StackBlock < '_ , A , R , Closure > { unsafe extern "C-unwind" fn clone_closure (dst : * mut c_void , src : * const c_void) { let dst : * mut Self = dst . cast () ; let src : * const Self = src . cast () ; let dst_closure = unsafe { ptr :: addr_of_mut ! ((* dst) . closure) } ; let src_closure = unsafe { & * ptr :: addr_of ! ((* src) . closure) } ; unsafe { ptr :: write (dst_closure , src_closure . clone ()) } ; } const DESCRIPTOR_WITH_CLONE : BlockDescriptorCopyDispose = BlockDescriptorCopyDispose { reserved : 0 , size : Self :: SIZE , copy : Some (Self :: clone_closure) , dispose : Some (Self :: drop_closure) , } ; }
};
}
