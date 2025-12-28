macro_rules! deps {
    () => {
        DebugLoc!();
        Section!();
        SectionId!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < R > Section < R > for DebugLoc < R > { fn id () -> SectionId { SectionId :: DebugLoc } fn reader (& self) -> & R { & self . section } }
    };
}

impl_444!();