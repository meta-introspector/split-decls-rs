// Generated macro for AttributeReader (struct)
macro_rules! Depcrate_read_elf_attributesAttributeReader {
() => {
// Module: crate::read::elf::attributes
// Provides: {"AttributeReader"}
// Dependencies: {}
# [doc = " A parser for the attributes in an [`AttributesSubsubsection`]."] # [doc = ""] # [doc = " The parser relies on the caller to know the format of the data for each attribute tag."] # [derive (Debug , Clone)] pub struct AttributeReader < 'data > { data : Bytes < 'data > , }
};
}
