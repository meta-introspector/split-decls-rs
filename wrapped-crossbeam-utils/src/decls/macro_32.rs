macro_rules! macro_32 {
    () => {
        # [cfg (not (target_has_atomic = "64"))] impl_arithmetic ! (u64 , fetch_update , "let a = AtomicCell::new(7u64);") ;
    };
}

macro_32!();