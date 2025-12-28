macro_rules! deps {
    () => {
        UnlockNotification!();
    };
}

macro_rules! unlock_notify_cb {
    () => {
        deps!();
        # [doc = " This function is an unlock-notify callback"] unsafe extern "C" fn unlock_notify_cb (ap_arg : * mut * mut c_void , n_arg : c_int) { use std :: slice :: from_raw_parts ; let args = from_raw_parts (ap_arg as * const & UnlockNotification , n_arg as usize) ; for un in args { drop (catch_unwind (std :: panic :: AssertUnwindSafe (| | un . fired ()))) ; } }
    };
}

unlock_notify_cb!();