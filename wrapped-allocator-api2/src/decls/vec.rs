macro_rules! vec {
    () => {
        # [cfg (feature = "alloc")] pub mod vec ;
    };
}

vec!()