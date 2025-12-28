macro_rules! macro_10 {
    () => {
        # [cfg (target_has_atomic = "16")] impl_borrow_decode ! (AtomicU16) ;
    };
}

macro_10!();