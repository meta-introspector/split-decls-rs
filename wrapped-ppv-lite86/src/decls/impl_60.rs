macro_rules! deps {
    () => {
        Swap64!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < W > Swap64 for x4 < W > where W : Swap64 + Copy , { fwd_unop_x4 ! (swap1) ; fwd_unop_x4 ! (swap2) ; fwd_unop_x4 ! (swap4) ; fwd_unop_x4 ! (swap8) ; fwd_unop_x4 ! (swap16) ; fwd_unop_x4 ! (swap32) ; fwd_unop_x4 ! (swap64) ; }
    };
}

impl_60!()