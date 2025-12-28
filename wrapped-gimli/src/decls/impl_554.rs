macro_rules! deps {
    () => {
        DebugRanges!();
        SectionId!();
        Section!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        impl < R > Section < R > for DebugRanges < R > { fn id () -> SectionId { SectionId :: DebugRanges } fn reader (& self) -> & R { & self . section } }
    };
}

impl_554!();