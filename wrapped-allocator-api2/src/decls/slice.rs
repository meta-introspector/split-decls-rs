macro_rules! slice {
    () => {
        # [cfg (feature = "alloc")] mod slice ;
    };
}

slice!()