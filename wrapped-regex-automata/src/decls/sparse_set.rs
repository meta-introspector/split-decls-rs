macro_rules! sparse_set {
    () => {
        # [cfg (feature = "alloc")] pub (crate) mod sparse_set ;
    };
}

sparse_set!();