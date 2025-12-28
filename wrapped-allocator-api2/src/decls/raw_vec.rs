macro_rules! raw_vec {
    () => {
        # [cfg (feature = "alloc")] mod raw_vec ;
    };
}

raw_vec!()