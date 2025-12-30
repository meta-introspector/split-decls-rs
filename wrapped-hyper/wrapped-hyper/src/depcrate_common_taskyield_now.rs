// Generated macro for yield_now (function)
macro_rules! Depcrate_common_taskyield_now {
() => {
// Module: crate::common::task
// Provides: {"yield_now"}
// Dependencies: {}
# [doc = " A function to help \"yield\" a future, such that it is re-scheduled immediately."] # [doc = ""] # [doc = " Useful for spin counts, so a future doesn't hog too much time."] pub (crate) fn yield_now (cx : & mut Context < '_ >) -> Poll < std :: convert :: Infallible > { cx . waker () . wake_by_ref () ; Poll :: Pending }
};
}
