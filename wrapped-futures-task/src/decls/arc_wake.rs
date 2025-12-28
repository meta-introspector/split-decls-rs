macro_rules! arc_wake {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] mod arc_wake ;
    };
}

arc_wake!();