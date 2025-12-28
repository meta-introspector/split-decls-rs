macro_rules! deps {
    () => {
        SectionId!();
        Section!();
        DebugStrOffsets!();
    };
}

macro_rules! impl_591 {
    () => {
        deps!();
        impl < R > Section < R > for DebugStrOffsets < R > { fn id () -> SectionId { SectionId :: DebugStrOffsets } fn reader (& self) -> & R { & self . section } }
    };
}

impl_591!();