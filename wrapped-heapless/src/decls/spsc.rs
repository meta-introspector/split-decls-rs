macro_rules! spsc {
    () => {
        # [cfg (any (feature = "portable-atomic" , target_has_atomic = "ptr" , has_atomic_load_store))] pub mod spsc ;
    };
}

spsc!();