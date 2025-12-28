macro_rules! ext_vec {
    () => {
        # [cfg (feature = "alloc")] mod ext_vec ;
    };
}

ext_vec!();