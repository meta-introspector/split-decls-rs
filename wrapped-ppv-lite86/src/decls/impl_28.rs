macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < W : Copy , G > MultiLane < [W ; 2] > for x2 < W , G > { # [inline (always)] fn to_lanes (self) -> [W ; 2] { self . 0 } # [inline (always)] fn from_lanes (lanes : [W ; 2]) -> Self { x2 :: new (lanes) } }
    };
}

impl_28!()