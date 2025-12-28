macro_rules! interpolate {
    () => {
        # [cfg (feature = "alloc")] pub mod interpolate ;
    };
}

interpolate!()