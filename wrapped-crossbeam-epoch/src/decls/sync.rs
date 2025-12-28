macro_rules! sync {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod sync ;
    };
}

sync!();