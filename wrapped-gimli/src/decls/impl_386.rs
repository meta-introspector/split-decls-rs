macro_rules! deps {
    () => {
        SectionId!();
        Section!();
        DebugTuIndex!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl < R > Section < R > for DebugTuIndex < R > { fn id () -> SectionId { SectionId :: DebugTuIndex } fn reader (& self) -> & R { & self . section } }
    };
}

impl_386!()