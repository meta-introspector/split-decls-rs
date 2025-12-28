macro_rules! deps {
    () => {
        DebugTypes!();
        SectionId!();
        Section!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < R > Section < R > for DebugTypes < R > { fn id () -> SectionId { SectionId :: DebugTypes } fn reader (& self) -> & R { & self . debug_types_section } }
    };
}

impl_657!()