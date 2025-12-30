// Generated macro for impl_78 (impl)
macro_rules! Depcrate_baseimpl_78 {
() => {
// Module: crate::base
// Provides: {"impl_78"}
// Dependencies: {}
unsafe impl < T : TCFType > FromMutVoid for T { unsafe fn from_mut_void < 'a > (x : * mut c_void) -> ItemMutRef < 'a , Self > { ItemMutRef (ManuallyDrop :: new (TCFType :: wrap_under_create_rule (T :: Ref :: from_void_ptr (x))) , PhantomData ,) } }
};
}
