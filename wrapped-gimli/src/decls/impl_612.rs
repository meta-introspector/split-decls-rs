macro_rules! deps {
    () => {
        DebugInfo!();
    };
}

macro_rules! impl_612 {
    () => {
        deps!();
        impl < R > From < R > for DebugInfo < R > { fn from (debug_info_section : R) -> Self { DebugInfo { debug_info_section } } }
    };
}

impl_612!()