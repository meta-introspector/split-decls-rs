macro_rules! serialize {
    () => {
        # [cfg (feature = "serialize")] pub mod serialize ;
    };
}

serialize!()