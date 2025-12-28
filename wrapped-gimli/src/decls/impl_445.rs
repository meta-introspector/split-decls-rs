macro_rules! deps {
    () => {
        DebugLoc!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl < R > From < R > for DebugLoc < R > { fn from (section : R) -> Self { DebugLoc { section } } }
    };
}

impl_445!();