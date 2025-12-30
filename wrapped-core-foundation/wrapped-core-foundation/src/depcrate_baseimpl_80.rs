// Generated macro for impl_80 (impl)
macro_rules! Depcrate_baseimpl_80 {
() => {
// Module: crate::base
// Provides: {"impl_80"}
// Dependencies: {}
unsafe impl FromVoid for u32 { unsafe fn from_void < 'a > (x : * const c_void) -> ItemRef < 'a , Self > { ItemRef (ManuallyDrop :: new (x as u32) , PhantomData) } }
};
}
