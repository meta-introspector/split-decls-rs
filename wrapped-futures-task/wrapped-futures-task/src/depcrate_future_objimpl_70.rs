// Generated macro for impl_70 (impl)
macro_rules! Depcrate_future_objimpl_70 {
() => {
// Module: crate::future_obj
// Provides: {"impl_70"}
// Dependencies: {}
unsafe impl < 'a , T > UnsafeFutureObj < 'a , T > for Pin < & 'a mut (dyn Future < Output = T > + 'a) > { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { unsafe { self . get_unchecked_mut () as * mut dyn Future < Output = T > } } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
};
}
