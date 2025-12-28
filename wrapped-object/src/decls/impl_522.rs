macro_rules! deps {
    () => {
        ReadRef!();
        FatHeader!();
        BigEndian!();
        Error!();
        MachOFatFile!();
        Result!();
        FatArch!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        impl < 'data , Fat : FatArch > MachOFatFile < 'data , Fat > { # [doc = " Attempt to parse the fat header and fat arches."] pub fn parse < R : ReadRef < 'data > > (data : R) -> Result < Self > { let mut offset = 0 ; let header = data . read :: < FatHeader > (& mut offset) . read_error ("Invalid fat header size or alignment") ? ; if header . magic . get (BigEndian) != Fat :: MAGIC { return Err (Error ("Invalid fat magic")) ; } let arches = data . read_slice :: < Fat > (& mut offset , header . nfat_arch . get (BigEndian) as usize) . read_error ("Invalid nfat_arch") ? ; Ok (MachOFatFile { header , arches }) } # [doc = " Return the fat header"] pub fn header (& self) -> & 'data macho :: FatHeader { self . header } # [doc = " Return the array of fat arches."] pub fn arches (& self) -> & 'data [Fat] { self . arches } }
    };
}

impl_522!()