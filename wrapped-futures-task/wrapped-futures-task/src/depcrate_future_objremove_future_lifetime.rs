// Generated macro for remove_future_lifetime (function)
macro_rules! Depcrate_future_objremove_future_lifetime {
() => {
// Module: crate::future_obj
// Provides: {"remove_future_lifetime"}
// Dependencies: {}
# [allow (single_use_lifetimes)] unsafe fn remove_future_lifetime < 'a , T > (ptr : * mut (dyn Future < Output = T > + 'a) ,) -> * mut (dyn Future < Output = T > + 'static) { unsafe { mem :: transmute (ptr) } }
};
}
