macro_rules! atomic {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod atomic ;
    };
}

atomic!()