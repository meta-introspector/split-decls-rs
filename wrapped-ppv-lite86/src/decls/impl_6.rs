macro_rules! deps {
    () => {
        RotateEachWord64!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < W , G > RotateEachWord64 for x2 < W , G > where W : Copy + RotateEachWord64 , { fwd_unop_x2 ! (rotate_each_word_right32) ; }
    };
}

impl_6!()