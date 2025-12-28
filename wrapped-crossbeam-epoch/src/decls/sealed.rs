macro_rules! sealed {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod sealed { pub trait Sealed { } }
    };
}

sealed!();