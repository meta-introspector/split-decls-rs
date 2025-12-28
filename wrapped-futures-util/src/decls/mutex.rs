macro_rules! mutex {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "std")] mod mutex ;
    };
}

mutex!();