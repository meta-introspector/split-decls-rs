macro_rules! futures_ordered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod futures_ordered ;
    };
}

futures_ordered!();