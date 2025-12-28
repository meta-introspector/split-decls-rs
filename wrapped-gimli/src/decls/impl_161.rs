macro_rules! deps {
    () => {
        DebugFrame!();
        Reader!();
        SectionId!();
        Section!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for DebugFrame < R > { fn id () -> SectionId { SectionId :: DebugFrame } fn reader (& self) -> & R { & self . section } }
    };
}

impl_161!()