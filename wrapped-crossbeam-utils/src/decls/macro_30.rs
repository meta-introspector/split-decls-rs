macro_rules! macro_30 {
    () => {
        # [cfg (target_has_atomic = "64")] impl_arithmetic ! (u64 , AtomicU64 , "let a = AtomicCell::new(7u64);") ;
    };
}

macro_30!();