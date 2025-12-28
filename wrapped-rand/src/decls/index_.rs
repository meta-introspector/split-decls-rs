macro_rules! index_ {
    () => {
        # [cfg (feature = "alloc")] # [path = "index.rs"] mod index_ ;
    };
}

index_!();