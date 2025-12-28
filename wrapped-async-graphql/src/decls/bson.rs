macro_rules! bson {
    () => {
        # [cfg (feature = "bson")] mod bson ;
    };
}

bson!()