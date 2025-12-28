macro_rules! try_flatten_unordered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod try_flatten_unordered ;
    };
}

try_flatten_unordered!()