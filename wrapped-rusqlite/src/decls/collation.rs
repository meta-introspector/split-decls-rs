macro_rules! collation {
    () => {
        # [cfg (feature = "collation")] mod collation ;
    };
}

collation!()