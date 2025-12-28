macro_rules! deps {
    () => {
        DebugLineStr!();
    };
}

macro_rules! impl_599 {
    () => {
        deps!();
        impl < R > From < R > for DebugLineStr < R > { fn from (section : R) -> Self { DebugLineStr { section } } }
    };
}

impl_599!();