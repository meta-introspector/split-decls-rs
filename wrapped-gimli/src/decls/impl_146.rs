macro_rules! deps {
    () => {
        Section!();
        SectionId!();
        DebugAddr!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < R > Section < R > for DebugAddr < R > { fn id () -> SectionId { SectionId :: DebugAddr } fn reader (& self) -> & R { & self . section } }
    };
}

impl_146!();