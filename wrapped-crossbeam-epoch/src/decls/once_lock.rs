macro_rules! once_lock {
    () => {
        # [cfg (feature = "std")] # [cfg (not (crossbeam_loom))] pub (crate) mod once_lock ;
    };
}

once_lock!()