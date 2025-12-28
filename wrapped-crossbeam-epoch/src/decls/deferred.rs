macro_rules! deferred {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod deferred ;
    };
}

deferred!()