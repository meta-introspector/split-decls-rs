macro_rules! arc {
    () => {
        # [cfg (any (feature = "portable-atomic" , target_has_atomic = "ptr"))] pub mod arc ;
    };
}

arc!();