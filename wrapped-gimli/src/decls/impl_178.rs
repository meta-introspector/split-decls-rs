macro_rules! deps {
    () => {
        SectionId!();
        Section!();
        EhFrame!();
        Reader!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for EhFrame < R > { fn id () -> SectionId { SectionId :: EhFrame } fn reader (& self) -> & R { & self . section } }
    };
}

impl_178!()