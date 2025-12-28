macro_rules! macro_63 {
    () => {
        # [cfg (any (target_has_atomic = "32" , not (target_pointer_width = "16")))] impl_atomic ! (AtomicU32 , u32) ;
    };
}

macro_63!()