// Generated macro for unlock_notify_cb (function)
macro_rules! Depcrate_unlock_notifyunlock_notify_cb {
() => {
// Module: crate::unlock_notify
// Provides: {"unlock_notify_cb"}
// Dependencies: {}
# [doc = " This function is an unlock-notify callback"] unsafe extern "C" fn unlock_notify_cb (ap_arg : * mut * mut c_void , n_arg : c_int) { use std :: slice :: from_raw_parts ; let args = from_raw_parts (ap_arg as * const & UnlockNotification , n_arg as usize) ; for un in args { drop (catch_unwind (std :: panic :: AssertUnwindSafe (| | un . fired ()))) ; } }
};
}
