macro_rules! macro_29 {
    () => {
        # [cfg (not (target_has_atomic = "32"))] impl_arithmetic ! (i32 , fetch_update , "let a = AtomicCell::new(7i32);") ;
    };
}

macro_29!();