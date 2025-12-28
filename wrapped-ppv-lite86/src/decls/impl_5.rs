macro_rules! deps {
    () => {
        RotateEachWord32!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < W , G > RotateEachWord32 for x2 < W , G > where W : Copy + RotateEachWord32 , { fwd_unop_x2 ! (rotate_each_word_right7) ; fwd_unop_x2 ! (rotate_each_word_right8) ; fwd_unop_x2 ! (rotate_each_word_right11) ; fwd_unop_x2 ! (rotate_each_word_right12) ; fwd_unop_x2 ! (rotate_each_word_right16) ; fwd_unop_x2 ! (rotate_each_word_right20) ; fwd_unop_x2 ! (rotate_each_word_right24) ; fwd_unop_x2 ! (rotate_each_word_right25) ; }
    };
}

impl_5!()