macro_rules! macro_13 {
    () => {
        # [cfg (target_has_atomic = "32")] impl_borrow_decode ! (AtomicU32) ;
    };
}

macro_13!();