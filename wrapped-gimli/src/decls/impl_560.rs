macro_rules! deps {
    () => {
        DebugRngLists!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        impl < R > From < R > for DebugRngLists < R > { fn from (section : R) -> Self { DebugRngLists { section } } }
    };
}

impl_560!();