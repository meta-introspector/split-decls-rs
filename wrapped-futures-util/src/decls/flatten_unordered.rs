macro_rules! flatten_unordered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] pub (crate) mod flatten_unordered ;
    };
}

flatten_unordered!()