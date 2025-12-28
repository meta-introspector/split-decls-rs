macro_rules! unified_diff {
    () => {
        # [cfg (feature = "unified_diff")] mod unified_diff ;
    };
}

unified_diff!();