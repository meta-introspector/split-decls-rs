macro_rules! deps {
    () => {
        PResult!();
    };
}

macro_rules! macro_19 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (PResult <'_ , () >, 24) ;
    };
}

macro_19!()