macro_rules! macro_31 {
    () => {
        # [cfg (target_has_atomic = "64")] impl_borrow_decode ! (AtomicI64) ;
    };
}

macro_31!();