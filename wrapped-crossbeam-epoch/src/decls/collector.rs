macro_rules! collector {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod collector ;
    };
}

collector!()