macro_rules! macro_65 {
    () => {
        # [cfg (any (target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,))] impl_atomic ! (AtomicU64 , u64) ;
    };
}

macro_65!();