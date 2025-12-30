// Generated macro for impl_652 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_652 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_652"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > AttributesSubsection < 'data , Elf > { # [doc = " Return the length of the attributes subsection."] pub fn length (& self) -> u32 { self . length } # [doc = " Return the vendor name of the attributes subsection."] pub fn vendor (& self) -> & 'data [u8] { self . vendor } # [doc = " Return an iterator over the sub-subsections."] pub fn subsubsections (& self) -> AttributesSubsubsectionIterator < 'data , Elf > { AttributesSubsubsectionIterator { endian : self . endian , data : self . data , } } }
};
}
