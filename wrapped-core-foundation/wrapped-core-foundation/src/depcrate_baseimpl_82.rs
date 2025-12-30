// Generated macro for impl_82 (impl)
macro_rules! Depcrate_baseimpl_82 {
() => {
// Module: crate::base
// Provides: {"impl_82"}
// Dependencies: {}
unsafe impl < T : TCFType > FromVoid for T { unsafe fn from_void < 'a > (x : * const c_void) -> ItemRef < 'a , Self > { ItemRef (ManuallyDrop :: new (TCFType :: wrap_under_create_rule (T :: Ref :: from_void_ptr (x))) , PhantomData ,) } }
};
}
