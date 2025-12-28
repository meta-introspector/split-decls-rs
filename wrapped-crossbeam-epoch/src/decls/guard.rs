macro_rules! guard {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod guard ;
    };
}

guard!();