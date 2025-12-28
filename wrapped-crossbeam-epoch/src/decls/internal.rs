macro_rules! internal {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod internal ;
    };
}

internal!();