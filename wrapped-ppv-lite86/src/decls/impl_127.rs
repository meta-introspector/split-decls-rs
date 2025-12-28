macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < S3 , S4 , NI > MultiLane < [u128 ; 1] > for u128x1_sse2 < S3 , S4 , NI > { # [inline (always)] fn to_lanes (self) -> [u128 ; 1] { unimplemented ! () } # [inline (always)] fn from_lanes (xs : [u128 ; 1]) -> Self { unimplemented ! ("{:?}" , xs) } }
    };
}

impl_127!()