macro_rules! trigger {
    () => {
        # [doc = " Trigger an interrupt, signalling to those checking for [`is_triggered()`] to stop what they are doing."] pub fn trigger () { IS_INTERRUPTED . store (true , Ordering :: SeqCst) ; }
    };
}

trigger!()