macro_rules! deps {
    () => {
        SectionId!();
        Section!();
        DebugMacro!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < R > Section < R > for DebugMacro < R > { fn id () -> SectionId { SectionId :: DebugMacro } fn reader (& self) -> & R { & self . section } }
    };
}

impl_491!();