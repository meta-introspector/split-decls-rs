macro_rules! deps {
    () => {
        DebugRanges!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        impl < R > From < R > for DebugRanges < R > { fn from (section : R) -> Self { DebugRanges { section } } }
    };
}

impl_555!()