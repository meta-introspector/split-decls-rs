macro_rules! sharded_lock {
    () => {
        # [cfg (not (crossbeam_loom))] mod sharded_lock ;
    };
}

sharded_lock!();