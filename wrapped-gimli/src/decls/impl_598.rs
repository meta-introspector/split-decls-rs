macro_rules! deps {
    () => {
        SectionId!();
        Section!();
        DebugLineStr!();
    };
}

macro_rules! impl_598 {
    () => {
        deps!();
        impl < R > Section < R > for DebugLineStr < R > { fn id () -> SectionId { SectionId :: DebugLineStr } fn reader (& self) -> & R { & self . section } }
    };
}

impl_598!()