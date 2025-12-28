macro_rules! deps {
    () => {
        ImageNtHeaders!();
        ReadRef!();
        ImageSectionHeader!();
        PeSegment!();
        PeFile!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > PeSegment < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [doc = " Get the PE file containing this segment."] pub fn pe_file (& self) -> & 'file PeFile < 'data , Pe , R > { self . file } # [doc = " Get the raw PE section header."] pub fn pe_section (& self) -> & 'data pe :: ImageSectionHeader { self . section } }
    };
}

impl_663!();