macro_rules! deadlock {
    () => {
        # [cfg (not (feature = "deadlock_detection"))] mod deadlock ;
    };
}

deadlock!();