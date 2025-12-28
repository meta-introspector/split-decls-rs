macro_rules! deps {
    () => {
        Result!();
        ReadRef!();
        CoffFile!();
        CoffHeader!();
        ImageSectionHeader!();
        CoffSegment!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffSegment < 'data , 'file , R , Coff > { # [doc = " Get the COFF file containing this segment."] pub fn coff_file (& self) -> & 'file CoffFile < 'data , R , Coff > { self . file } # [doc = " Get the raw COFF section header."] pub fn coff_section (& self) -> & 'data pe :: ImageSectionHeader { self . section } fn bytes (& self) -> Result < & 'data [u8] > { self . section . coff_data (self . file . data) . read_error ("Invalid COFF section offset or size") } }
    };
}

impl_212!()