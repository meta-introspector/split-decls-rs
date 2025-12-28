macro_rules! deps {
    () => {
        Section!();
        SectionId!();
        DebugStr!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl < R > Section < R > for DebugStr < R > { fn id () -> SectionId { SectionId :: DebugStr } fn reader (& self) -> & R { & self . debug_str_section } }
    };
}

impl_586!()