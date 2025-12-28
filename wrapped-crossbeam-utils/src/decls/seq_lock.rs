macro_rules! seq_lock {
    () => {
        # [cfg (target_has_atomic = "ptr")] # [cfg (not (crossbeam_loom))] # [cfg_attr (any (target_pointer_width = "16" , target_pointer_width = "32") , path = "seq_lock_wide.rs")] mod seq_lock ;
    };
}

seq_lock!()