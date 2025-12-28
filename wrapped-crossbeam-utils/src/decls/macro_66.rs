macro_rules! macro_66 {
    () => {
        # [cfg (any (target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,))] impl_atomic ! (AtomicI64 , i64) ;
    };
}

macro_66!()