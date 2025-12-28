macro_rules! try_for_each_concurrent {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod try_for_each_concurrent ;
    };
}

try_for_each_concurrent!()