macro_rules! macro_28 {
    () => {
        # [cfg (not (target_has_atomic = "32"))] impl_arithmetic ! (u32 , fetch_update , "let a = AtomicCell::new(7u32);") ;
    };
}

macro_28!();