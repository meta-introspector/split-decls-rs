// Generated macro for remove_drop_lifetime (function)
macro_rules! Depcrate_future_objremove_drop_lifetime {
() => {
// Module: crate::future_obj
// Provides: {"remove_drop_lifetime"}
// Dependencies: {}
# [allow (single_use_lifetimes)] unsafe fn remove_drop_lifetime < 'a , T > (ptr : unsafe fn (* mut (dyn Future < Output = T > + 'a)) ,) -> unsafe fn (* mut (dyn Future < Output = T > + 'static)) { unsafe { mem :: transmute (ptr) } }
};
}
