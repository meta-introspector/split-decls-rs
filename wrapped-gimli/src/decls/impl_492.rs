macro_rules! deps {
    () => {
        DebugMacro!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl < R > From < R > for DebugMacro < R > { fn from (macro_section : R) -> Self { DebugMacro { section : macro_section , } } }
    };
}

impl_492!()