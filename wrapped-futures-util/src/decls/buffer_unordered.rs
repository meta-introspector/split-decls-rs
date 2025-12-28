macro_rules! buffer_unordered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod buffer_unordered ;
    };
}

buffer_unordered!();