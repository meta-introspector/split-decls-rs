macro_rules! deps {
    () => {
        Section!();
        SectionId!();
        DebugMacro!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < R > Section < R > for DebugMacro < R > { fn id () -> SectionId { SectionId :: DebugMacro } fn reader (& self) -> & R { & self . section } }
    };
}

impl_491!()