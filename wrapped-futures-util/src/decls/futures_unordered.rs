macro_rules! futures_unordered {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] pub mod futures_unordered ;
    };
}

futures_unordered!();