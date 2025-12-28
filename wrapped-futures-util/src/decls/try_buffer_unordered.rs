macro_rules! try_buffer_unordered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod try_buffer_unordered ;
    };
}

try_buffer_unordered!();