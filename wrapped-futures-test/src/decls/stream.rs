macro_rules! stream {
    () => {
        # [cfg (feature = "std")] pub mod stream ;
    };
}

stream!()