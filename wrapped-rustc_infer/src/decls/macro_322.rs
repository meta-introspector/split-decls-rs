macro_rules! deps {
    () => {
        PredicateObligation!();
    };
}

macro_rules! macro_322 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (PredicateObligation <'_ >, 48) ;
    };
}

macro_322!();