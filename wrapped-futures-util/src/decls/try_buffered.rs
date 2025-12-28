macro_rules! try_buffered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod try_buffered ;
    };
}

try_buffered!();