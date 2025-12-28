macro_rules! macro_26 {
    () => {
        # [cfg (target_has_atomic = "32")] impl_arithmetic ! (u32 , AtomicU32 , "let a = AtomicCell::new(7u32);") ;
    };
}

macro_26!()