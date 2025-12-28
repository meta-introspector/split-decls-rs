macro_rules! deps {
    () => {
        Reader!();
        Section!();
        DebugPubTypes!();
        SectionId!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for DebugPubTypes < R > { fn id () -> SectionId { SectionId :: DebugPubTypes } fn reader (& self) -> & R { self . 0 . reader () } }
    };
}

impl_545!();