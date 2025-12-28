macro_rules! macro_31 {
    () => {
        # [cfg (target_has_atomic = "64")] impl_arithmetic ! (i64 , AtomicI64 , "let a = AtomicCell::new(7i64);") ;
    };
}

macro_31!();