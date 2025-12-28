macro_rules! deps {
    () => {
        SubregionOrigin!();
    };
}

macro_rules! macro_258 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (SubregionOrigin <'_ >, 32) ;
    };
}

macro_258!();