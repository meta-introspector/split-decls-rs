macro_rules! deps {
    () => {
        DebugLocLists!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl < R > From < R > for DebugLocLists < R > { fn from (section : R) -> Self { DebugLocLists { section } } }
    };
}

impl_450!();