macro_rules! writer {
    () => {
        # [cfg (feature = "encoder")] mod writer ;
    };
}

writer!()