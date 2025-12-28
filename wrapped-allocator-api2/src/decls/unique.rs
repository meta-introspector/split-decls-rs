macro_rules! unique {
    () => {
        # [cfg (feature = "alloc")] mod unique ;
    };
}

unique!()