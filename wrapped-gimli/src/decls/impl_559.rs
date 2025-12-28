macro_rules! deps {
    () => {
        Section!();
        SectionId!();
        DebugRngLists!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl < R > Section < R > for DebugRngLists < R > { fn id () -> SectionId { SectionId :: DebugRngLists } fn reader (& self) -> & R { & self . section } }
    };
}

impl_559!()