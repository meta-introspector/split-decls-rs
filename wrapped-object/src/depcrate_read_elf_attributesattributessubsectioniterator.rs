// Generated macro for AttributesSubsectionIterator (struct)
macro_rules! Depcrate_read_elf_attributesAttributesSubsectionIterator {
() => {
// Module: crate::read::elf::attributes
// Provides: {"AttributesSubsectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the subsections in an [`AttributesSection`]."] # [derive (Debug , Clone)] pub struct AttributesSubsectionIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
};
}
