macro_rules! try_join_all {
    () => {
        # [cfg (feature = "alloc")] mod try_join_all ;
    };
}

try_join_all!()