macro_rules! deps {
    () => {
        DebugPubTypes!();
        SectionId!();
        Section!();
        Reader!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for DebugPubTypes < R > { fn id () -> SectionId { SectionId :: DebugPubTypes } fn reader (& self) -> & R { self . 0 . reader () } }
    };
}

impl_545!()