macro_rules! select_all {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] pub mod select_all ;
    };
}

select_all!();