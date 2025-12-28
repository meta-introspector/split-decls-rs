macro_rules! deps {
    () => {
        Swap64!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < W , G > Swap64 for x2 < W , G > where W : Swap64 + Copy , { fwd_unop_x2 ! (swap1) ; fwd_unop_x2 ! (swap2) ; fwd_unop_x2 ! (swap4) ; fwd_unop_x2 ! (swap8) ; fwd_unop_x2 ! (swap16) ; fwd_unop_x2 ! (swap32) ; fwd_unop_x2 ! (swap64) ; }
    };
}

impl_27!();