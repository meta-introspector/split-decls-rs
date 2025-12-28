macro_rules! sink {
    () => {
        # [cfg (feature = "std")] pub mod sink ;
    };
}

sink!()