macro_rules! deps {
    () => {
        Section!();
        DebugCuIndex!();
        SectionId!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < R > Section < R > for DebugCuIndex < R > { fn id () -> SectionId { SectionId :: DebugCuIndex } fn reader (& self) -> & R { & self . section } }
    };
}

impl_380!();