macro_rules! deps {
    () => {
        Section!();
        DebugAranges!();
        SectionId!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl < R > Section < R > for DebugAranges < R > { fn id () -> SectionId { SectionId :: DebugAranges } fn reader (& self) -> & R { & self . section } }
    };
}

impl_363!();