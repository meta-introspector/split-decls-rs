// Generated macro for Pool (struct)
macro_rules! Depcrate_rc_autoreleasePool {
() => {
// Module: crate::rc::autorelease
// Provides: {"Pool"}
// Dependencies: {}
# [doc = " The actual pool object."] # [doc = ""] # [doc = " It is drained when dropped."] # [doc = ""] # [doc = " This is not [`Send`], since `objc_autoreleasePoolPop` must be called on"] # [doc = " the same thread as `objc_autoreleasePoolPush`."] # [doc = ""] # [doc = " And this is not [`Sync`], since that would make `AutoreleasePool` `Send`."] # [derive (Debug)] struct Pool { # [doc = " This is an opaque handle, and is not guaranteed to be neither a valid"] # [doc = " nor an aligned pointer."] context : * mut c_void , }
};
}
