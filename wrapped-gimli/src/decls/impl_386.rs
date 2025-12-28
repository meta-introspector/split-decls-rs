macro_rules! deps {
    () => {
        Section!();
        DebugTuIndex!();
        SectionId!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl < R > Section < R > for DebugTuIndex < R > { fn id () -> SectionId { SectionId :: DebugTuIndex } fn reader (& self) -> & R { & self . section } }
    };
}

impl_386!();