// Generated macro for impl_135 (impl)
macro_rules! Depcrate_stackimpl_135 {
() => {
// Module: crate::stack
// Provides: {"impl_135"}
// Dependencies: {}
impl < A , R , Closure > StackBlock < '_ , A , R , Closure > { # [doc = " The size of the block header and the trailing closure."] # [doc = ""] # [doc = " This ensures that the closure that the block contains is also moved to"] # [doc = " the heap in `_Block_copy` operations."] const SIZE : c_ulong = mem :: size_of :: < Self > () as _ ; unsafe extern "C-unwind" fn drop_closure (block : * mut c_void) { let block : * mut Self = block . cast () ; let closure = unsafe { ptr :: addr_of_mut ! ((* block) . closure) } ; unsafe { ptr :: drop_in_place (closure) } ; } const DESCRIPTOR_BASIC : BlockDescriptor = BlockDescriptor { reserved : 0 , size : Self :: SIZE , } ; }
};
}
