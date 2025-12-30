// Generated macro for impl_121 (impl)
macro_rules! Depcrate_boxedimpl_121 {
() => {
// Module: crate::boxed
// Provides: {"impl_121"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for Box < T , A > { # [inline] fn drop (& mut self) { let ptr = self . 0 ; unsafe { let layout = Layout :: for_value_raw (ptr . as_ptr ()) ; if layout . size () != 0 { self . 1 . deallocate (From :: from (ptr . cast ()) , layout) ; } } } }
};
}
