macro_rules! xcoff {
    () => {
        # [cfg (feature = "xcoff")] pub mod xcoff ;
    };
}

xcoff!()