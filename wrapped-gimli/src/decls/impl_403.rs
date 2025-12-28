macro_rules! deps {
    () => {
        Section!();
        DebugLine!();
        SectionId!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < R > Section < R > for DebugLine < R > { fn id () -> SectionId { SectionId :: DebugLine } fn reader (& self) -> & R { & self . debug_line_section } }
    };
}

impl_403!();