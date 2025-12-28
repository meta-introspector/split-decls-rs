macro_rules! deps {
    () => {
        DebugStrOffsets!();
    };
}

macro_rules! impl_592 {
    () => {
        deps!();
        impl < R > From < R > for DebugStrOffsets < R > { fn from (section : R) -> Self { DebugStrOffsets { section } } }
    };
}

impl_592!();