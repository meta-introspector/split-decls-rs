macro_rules! deps {
    () => {
        CovmapVersion!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl CovmapVersion { fn to_u32 (self) -> u32 { self as u32 } }
    };
}

impl_258!();