macro_rules! macro_25 {
    () => {
        # [cfg (target_has_atomic = "16")] impl_borrow_decode ! (AtomicI16) ;
    };
}

macro_25!();