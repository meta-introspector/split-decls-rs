macro_rules! join_all {
    () => {
        # [cfg (feature = "alloc")] mod join_all ;
    };
}

join_all!();