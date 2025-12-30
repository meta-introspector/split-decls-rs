// Generated macro for AttributesSubsubsectionIterator (struct)
macro_rules! Depcrate_read_elf_attributesAttributesSubsubsectionIterator {
() => {
// Module: crate::read::elf::attributes
// Provides: {"AttributesSubsubsectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sub-subsections in an [`AttributesSubsection`]."] # [derive (Debug , Clone)] pub struct AttributesSubsubsectionIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
};
}
