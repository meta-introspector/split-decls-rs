// Generated macro for AttributesSubsection (struct)
macro_rules! Depcrate_read_elf_attributesAttributesSubsection {
() => {
// Module: crate::read::elf::attributes
// Provides: {"AttributesSubsection"}
// Dependencies: {}
# [doc = " A subsection in an [`AttributesSection`]."] # [doc = ""] # [doc = " A subsection is identified by a vendor name.  It contains a series of"] # [doc = " [`AttributesSubsubsection`]."] # [derive (Debug , Clone)] pub struct AttributesSubsection < 'data , Elf : FileHeader > { endian : Elf :: Endian , length : u32 , vendor : & 'data [u8] , data : Bytes < 'data > , }
};
}
