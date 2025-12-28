macro_rules! FEATURE_TEST_ACTIVATED {
    () => {
        # [cfg (feature = "auto")] pub const FEATURE_TEST_ACTIVATED : bool = cfg ! (feature = "test") ;
    };
}

FEATURE_TEST_ACTIVATED!()