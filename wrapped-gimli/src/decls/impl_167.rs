macro_rules! deps {
    () => {
        SectionId!();
        Reader!();
        EhFrameHdr!();
        Section!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < R : Reader > Section < R > for EhFrameHdr < R > { fn id () -> SectionId { SectionId :: EhFrameHdr } fn reader (& self) -> & R { & self . 0 } }
    };
}

impl_167!()