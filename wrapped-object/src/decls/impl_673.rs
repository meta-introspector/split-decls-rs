macro_rules! deps {
    () => {
        PeSection!();
        ImageNtHeaders!();
        ReadRef!();
        ImageSectionHeader!();
        PeFile!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > PeSection < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [doc = " Get the PE file containing this segment."] pub fn pe_file (& self) -> & 'file PeFile < 'data , Pe , R > { self . file } # [doc = " Get the raw PE section header."] pub fn pe_section (& self) -> & 'data pe :: ImageSectionHeader { self . section } }
    };
}

impl_673!()