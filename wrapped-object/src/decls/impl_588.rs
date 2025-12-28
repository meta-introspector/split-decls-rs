macro_rules! deps {
    () => {
        MachOSection!();
        MachOFile!();
        CompressedFileRange!();
        MachHeader!();
        Result!();
        Section!();
        Relocation!();
        Endian!();
        ReadRef!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > MachOSection < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { # [doc = " Get the Mach-O file containing this section."] pub fn macho_file (& self) -> & 'file MachOFile < 'data , Mach , R > { self . file } # [doc = " Get the raw Mach-O section structure."] pub fn macho_section (& self) -> & 'data Mach :: Section { self . internal . section } # [doc = " Get the raw Mach-O relocation entries."] pub fn macho_relocations (& self) -> Result < & 'data [macho :: Relocation < Mach :: Endian >] > { self . internal . section . relocations (self . file . endian , self . internal . data) } fn bytes (& self) -> Result < & 'data [u8] > { self . internal . section . data (self . file . endian , self . internal . data) . read_error ("Invalid Mach-O section size or offset") } fn maybe_compressed_gnu (& self) -> Result < Option < CompressedFileRange > > { if ! self . name () . map_or (false , | name | name . starts_with ("__zdebug_")) { return Ok (None) ; } let (section_offset , section_size) = self . file_range () . read_error ("Invalid ELF GNU compressed section type") ? ; gnu_compression :: compressed_file_range (self . internal . data , section_offset , section_size) . map (Some) } }
    };
}

impl_588!()