macro_rules! deps {
    () => {
        DebugMacinfo!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl < R > From < R > for DebugMacinfo < R > { fn from (macinfo_section : R) -> Self { DebugMacinfo { section : macinfo_section , } } }
    };
}

impl_486!()