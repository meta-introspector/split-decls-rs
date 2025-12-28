macro_rules! deps {
    () => {
        SectionId!();
        DebugMacinfo!();
        Section!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl < R > Section < R > for DebugMacinfo < R > { fn id () -> SectionId { SectionId :: DebugMacinfo } fn reader (& self) -> & R { & self . section } }
    };
}

impl_485!();