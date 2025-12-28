macro_rules! atomic_cell {
    () => {
        # [cfg (target_has_atomic = "ptr")] # [cfg (not (crossbeam_loom))] mod atomic_cell ;
    };
}

atomic_cell!();