macro_rules! vec {
    () => {
        # [cfg (feature = "alloc")] pub (crate) mod vec ;
    };
}

vec!();