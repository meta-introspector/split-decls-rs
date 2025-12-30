// Generated macro for impl_68 (impl)
macro_rules! Depcrate_future_objimpl_68 {
() => {
// Module: crate::future_obj
// Provides: {"impl_68"}
// Dependencies: {}
unsafe impl < 'a , T > UnsafeFutureObj < 'a , T > for & 'a mut (dyn Future < Output = T > + Unpin + 'a) { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { self as * mut dyn Future < Output = T > } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
};
}
