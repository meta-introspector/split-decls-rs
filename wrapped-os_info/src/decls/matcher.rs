macro_rules! matcher {
    () => {
        # [cfg (not (windows))] mod matcher ;
    };
}

matcher!();