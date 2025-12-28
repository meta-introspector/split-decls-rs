macro_rules! macro_16 {
    () => {
        # [cfg (target_has_atomic = "64")] impl_borrow_decode ! (AtomicU64) ;
    };
}

macro_16!()