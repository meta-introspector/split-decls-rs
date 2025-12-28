macro_rules! deps {
    () => {
        DebugAranges!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl < R > From < R > for DebugAranges < R > { fn from (section : R) -> Self { DebugAranges { section } } }
    };
}

impl_364!()