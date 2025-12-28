macro_rules! deps {
    () => {
        Hunk!();
        Side!();
    };
}

macro_rules! range_by_side {
    () => {
        deps!();
        fn range_by_side (hunk : & mut Hunk) -> & mut Range < u32 > { match hunk . side { Side :: Current | Side :: Other => & mut hunk . after , Side :: Ancestor => & mut hunk . before , } }
    };
}

range_by_side!();