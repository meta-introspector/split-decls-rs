macro_rules! HYPER_POLL_PENDING {
    () => {
        # [doc = " Return in a poll function to indicate it is still pending."] # [doc = ""] # [doc = " The passed in `hyper_waker` should be registered to wake up the task at"] # [doc = " some later point."] pub const HYPER_POLL_PENDING : c_int = 1 ;
    };
}

HYPER_POLL_PENDING!();