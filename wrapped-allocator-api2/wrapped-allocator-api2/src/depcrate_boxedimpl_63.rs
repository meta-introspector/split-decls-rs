// Generated macro for impl_63 (impl)
macro_rules! Depcrate_boxedimpl_63 {
() => {
// Module: crate::boxed
// Provides: {"impl_63"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > Drop for Box < T , A > { # [inline (always)] fn drop (& mut self) { let layout = Layout :: for_value :: < T > (& * * self) ; unsafe { ptr :: drop_in_place (self . 0 . as_mut ()) ; self . 1 . deallocate (self . 0 . as_non_null_ptr () . cast () , layout) ; } } }
};
}
