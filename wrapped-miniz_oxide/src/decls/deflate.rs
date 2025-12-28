macro_rules! deflate {
    () => {
        # [cfg (feature = "with-alloc")] pub mod deflate ;
    };
}

deflate!()