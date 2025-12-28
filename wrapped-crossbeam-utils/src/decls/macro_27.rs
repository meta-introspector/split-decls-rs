macro_rules! macro_27 {
    () => {
        # [cfg (target_has_atomic = "32")] impl_arithmetic ! (i32 , AtomicI32 , "let a = AtomicCell::new(7i32);") ;
    };
}

macro_27!()