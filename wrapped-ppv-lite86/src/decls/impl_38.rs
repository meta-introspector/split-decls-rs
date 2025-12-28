macro_rules! deps {
    () => {
        RotateEachWord64!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < W > RotateEachWord64 for x4 < W > where W : Copy + RotateEachWord64 , { fwd_unop_x4 ! (rotate_each_word_right32) ; }
    };
}

impl_38!()