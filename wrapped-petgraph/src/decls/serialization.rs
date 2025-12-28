macro_rules! serialization {
    () => {
        # [cfg (feature = "serde-1")] mod serialization ;
    };
}

serialization!()