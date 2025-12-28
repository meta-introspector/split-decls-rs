macro_rules! oneshot {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] pub mod oneshot ;
    };
}

oneshot!()