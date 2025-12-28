macro_rules! deps {
    () => {
        RotateEachWord32!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < W > RotateEachWord32 for x4 < W > where W : Copy + RotateEachWord32 , { fwd_unop_x4 ! (rotate_each_word_right7) ; fwd_unop_x4 ! (rotate_each_word_right8) ; fwd_unop_x4 ! (rotate_each_word_right11) ; fwd_unop_x4 ! (rotate_each_word_right12) ; fwd_unop_x4 ! (rotate_each_word_right16) ; fwd_unop_x4 ! (rotate_each_word_right20) ; fwd_unop_x4 ! (rotate_each_word_right24) ; fwd_unop_x4 ! (rotate_each_word_right25) ; }
    };
}

impl_37!();