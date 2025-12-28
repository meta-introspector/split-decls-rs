macro_rules! buffered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod buffered ;
    };
}

buffered!();