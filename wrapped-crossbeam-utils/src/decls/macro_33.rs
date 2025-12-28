macro_rules! macro_33 {
    () => {
        # [cfg (not (target_has_atomic = "64"))] impl_arithmetic ! (i64 , fetch_update , "let a = AtomicCell::new(7i64);") ;
    };
}

macro_33!();