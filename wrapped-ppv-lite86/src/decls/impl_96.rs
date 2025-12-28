macro_rules! deps {
    () => {
        MultiLane!();
        VZip!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < V , T > VZip < V > for T where V : MultiLane < T > , { # [inline (always)] fn vzip (self) -> V { V :: from_lanes (self) } }
    };
}

impl_96!()