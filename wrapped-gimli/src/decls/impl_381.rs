macro_rules! deps {
    () => {
        DebugCuIndex!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl < R > From < R > for DebugCuIndex < R > { fn from (section : R) -> Self { DebugCuIndex { section } } }
    };
}

impl_381!()