macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl MultiLane < [u128 ; 1] > for u128x1_generic { # [inline (always)] fn to_lanes (self) -> [u128 ; 1] { self . 0 } # [inline (always)] fn from_lanes (xs : [u128 ; 1]) -> Self { Self (xs) } }
    };
}

impl_338!();