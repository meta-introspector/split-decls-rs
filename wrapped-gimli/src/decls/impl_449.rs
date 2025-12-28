macro_rules! deps {
    () => {
        DebugLocLists!();
        Section!();
        SectionId!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl < R > Section < R > for DebugLocLists < R > { fn id () -> SectionId { SectionId :: DebugLocLists } fn reader (& self) -> & R { & self . section } }
    };
}

impl_449!()