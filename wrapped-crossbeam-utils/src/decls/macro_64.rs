macro_rules! macro_64 {
    () => {
        # [cfg (any (target_has_atomic = "32" , not (target_pointer_width = "16")))] impl_atomic ! (AtomicI32 , i32) ;
    };
}

macro_64!();