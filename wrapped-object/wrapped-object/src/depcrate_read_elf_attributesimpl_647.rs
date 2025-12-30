// Generated macro for impl_647 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_647 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_647"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > AttributesSection < 'data , Elf > { # [doc = " Parse an ELF attributes section given the section data."] pub fn new (endian : Elf :: Endian , data : & 'data [u8]) -> Result < Self > { let mut data = Bytes (data) ; let version = data . read :: < u8 > () . cloned () . unwrap_or (b'A') ; Ok (AttributesSection { endian , version , data , }) } # [doc = " Return the version of the attributes section."] pub fn version (& self) -> u8 { self . version } # [doc = " Return an iterator over the subsections."] pub fn subsections (& self) -> Result < AttributesSubsectionIterator < 'data , Elf > > { if self . version != b'A' { return Err (Error ("Unsupported ELF attributes section version")) ; } Ok (AttributesSubsectionIterator { endian : self . endian , data : self . data , }) } }
};
}
