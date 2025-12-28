macro_rules! deps {
    () => {
        DebugTuIndex!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl < R > From < R > for DebugTuIndex < R > { fn from (section : R) -> Self { DebugTuIndex { section } } }
    };
}

impl_387!();