macro_rules! property_bool {
    () => {
        # [cfg (feature = "unicode-bool")] pub mod property_bool ;
    };
}

property_bool!()