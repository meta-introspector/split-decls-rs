macro_rules! deps {
    () => {
        Reader!();
        Section!();
        DebugPubNames!();
        SectionId!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for DebugPubNames < R > { fn id () -> SectionId { SectionId :: DebugPubNames } fn reader (& self) -> & R { self . 0 . reader () } }
    };
}

impl_533!()