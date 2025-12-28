macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl MultiLane < [u64 ; 2] > for u64x2_generic { # [inline (always)] fn to_lanes (self) -> [u64 ; 2] { self . 0 } # [inline (always)] fn from_lanes (xs : [u64 ; 2]) -> Self { Self (xs) } }
    };
}

impl_336!()