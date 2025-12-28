macro_rules! macro_4 {
    () => {
        # [cfg (target_has_atomic = "8")] impl_borrow_decode ! (AtomicBool) ;
    };
}

macro_4!();