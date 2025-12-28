macro_rules! rw_lock {
    () => {
        # [cfg (feature = "internal-test-strategies")] mod rw_lock ;
    };
}

rw_lock!()