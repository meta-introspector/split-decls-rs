macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl MultiLane < [u32 ; 4] > for u32x4_generic { # [inline (always)] fn to_lanes (self) -> [u32 ; 4] { self . 0 } # [inline (always)] fn from_lanes (xs : [u32 ; 4]) -> Self { Self (xs) } }
    };
}

impl_335!()