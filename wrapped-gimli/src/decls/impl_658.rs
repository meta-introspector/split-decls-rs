macro_rules! deps {
    () => {
        DebugTypes!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl < R > From < R > for DebugTypes < R > { fn from (debug_types_section : R) -> Self { DebugTypes { debug_types_section , } } }
    };
}

impl_658!()