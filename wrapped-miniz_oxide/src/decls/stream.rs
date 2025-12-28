macro_rules! stream {
    () => {
        # [cfg (not (feature = "rustc-dep-of-std"))] pub mod stream ;
    };
}

stream!();