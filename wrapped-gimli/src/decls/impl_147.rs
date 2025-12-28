macro_rules! deps {
    () => {
        DebugAddr!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < R > From < R > for DebugAddr < R > { fn from (section : R) -> Self { DebugAddr { section } } }
    };
}

impl_147!();