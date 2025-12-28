macro_rules! deps {
    () => {
        DebugStr!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl < R > From < R > for DebugStr < R > { fn from (debug_str_section : R) -> Self { DebugStr { debug_str_section } } }
    };
}

impl_587!()