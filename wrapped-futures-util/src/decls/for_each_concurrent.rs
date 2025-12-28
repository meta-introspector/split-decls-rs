macro_rules! for_each_concurrent {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod for_each_concurrent ;
    };
}

for_each_concurrent!()