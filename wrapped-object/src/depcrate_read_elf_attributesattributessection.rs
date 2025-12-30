// Generated macro for AttributesSection (struct)
macro_rules! Depcrate_read_elf_attributesAttributesSection {
() => {
// Module: crate::read::elf::attributes
// Provides: {"AttributesSection"}
// Dependencies: {}
# [doc = " An ELF attributes section."] # [doc = ""] # [doc = " This may be a GNU attributes section, or an architecture specific attributes section."] # [doc = ""] # [doc = " An attributes section contains a series of [`AttributesSubsection`]."] # [doc = ""] # [doc = " Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)"] # [doc = " and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes)."] # [derive (Debug , Clone)] pub struct AttributesSection < 'data , Elf : FileHeader > { endian : Elf :: Endian , version : u8 , data : Bytes < 'data > , }
};
}
