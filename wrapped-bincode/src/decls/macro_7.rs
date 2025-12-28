macro_rules! macro_7 {
    () => {
        # [cfg (target_has_atomic = "8")] impl_borrow_decode ! (AtomicU8) ;
    };
}

macro_7!();