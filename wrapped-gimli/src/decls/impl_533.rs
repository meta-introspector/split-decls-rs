macro_rules! deps {
    () => {
        DebugPubNames!();
        SectionId!();
        Section!();
        Reader!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for DebugPubNames < R > { fn id () -> SectionId { SectionId :: DebugPubNames } fn reader (& self) -> & R { self . 0 . reader () } }
    };
}

impl_533!();