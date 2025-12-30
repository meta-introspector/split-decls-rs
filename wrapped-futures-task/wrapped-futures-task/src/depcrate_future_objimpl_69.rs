// Generated macro for impl_69 (impl)
macro_rules! Depcrate_future_objimpl_69 {
() => {
// Module: crate::future_obj
// Provides: {"impl_69"}
// Dependencies: {}
unsafe impl < 'a , T , F > UnsafeFutureObj < 'a , T > for Pin < & 'a mut F > where F : Future < Output = T > + 'a , { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { unsafe { self . get_unchecked_mut () as * mut dyn Future < Output = T > } } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
};
}
