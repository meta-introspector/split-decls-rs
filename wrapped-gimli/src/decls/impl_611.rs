macro_rules! deps {
    () => {
        SectionId!();
        DebugInfo!();
        Section!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl < R > Section < R > for DebugInfo < R > { fn id () -> SectionId { SectionId :: DebugInfo } fn reader (& self) -> & R { & self . debug_info_section } }
    };
}

impl_611!();