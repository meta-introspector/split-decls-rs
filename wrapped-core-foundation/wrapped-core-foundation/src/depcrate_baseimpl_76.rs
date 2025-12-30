// Generated macro for impl_76 (impl)
macro_rules! Depcrate_baseimpl_76 {
() => {
// Module: crate::base
// Provides: {"impl_76"}
// Dependencies: {}
unsafe impl FromMutVoid for u32 { unsafe fn from_mut_void < 'a > (x : * mut c_void) -> ItemMutRef < 'a , Self > { ItemMutRef (ManuallyDrop :: new (x as u32) , PhantomData) } }
};
}
