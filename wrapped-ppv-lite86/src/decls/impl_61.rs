macro_rules! deps {
    () => {
        MultiLane!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < W : Copy > MultiLane < [W ; 4] > for x4 < W > { # [inline (always)] fn to_lanes (self) -> [W ; 4] { self . 0 } # [inline (always)] fn from_lanes (lanes : [W ; 4]) -> Self { x4 (lanes) } }
    };
}

impl_61!()