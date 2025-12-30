// Generated macro for impl_67 (impl)
macro_rules! Depcrate_future_objimpl_67 {
() => {
// Module: crate::future_obj
// Provides: {"impl_67"}
// Dependencies: {}
unsafe impl < 'a , T , F > UnsafeFutureObj < 'a , T > for & 'a mut F where F : Future < Output = T > + Unpin + 'a , { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { self as * mut dyn Future < Output = T > } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
};
}
