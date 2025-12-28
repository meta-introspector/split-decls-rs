macro_rules! general_category {
    () => {
        # [cfg (feature = "unicode-gencat")] pub mod general_category ;
    };
}

general_category!()