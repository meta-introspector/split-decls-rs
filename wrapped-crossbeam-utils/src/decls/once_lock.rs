macro_rules! once_lock {
    () => {
        # [cfg (not (crossbeam_loom))] mod once_lock ;
    };
}

once_lock!();