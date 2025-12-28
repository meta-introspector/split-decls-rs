macro_rules! hybrid {
    () => {
        # [cfg (feature = "hybrid")] pub mod hybrid ;
    };
}

hybrid!()