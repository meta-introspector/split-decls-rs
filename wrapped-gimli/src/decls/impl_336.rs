macro_rules! deps {
    () => {
        Section!();
        SectionId!();
        DebugAbbrev!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl < R > Section < R > for DebugAbbrev < R > { fn id () -> SectionId { SectionId :: DebugAbbrev } fn reader (& self) -> & R { & self . debug_abbrev_section } }
    };
}

impl_336!();