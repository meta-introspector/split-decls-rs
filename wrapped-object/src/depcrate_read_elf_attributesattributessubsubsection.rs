// Generated macro for AttributesSubsubsection (struct)
macro_rules! Depcrate_read_elf_attributesAttributesSubsubsection {
() => {
// Module: crate::read::elf::attributes
// Provides: {"AttributesSubsubsection"}
// Dependencies: {}
# [doc = " A sub-subsection in an [`AttributesSubsection`]."] # [doc = ""] # [doc = " A sub-subsection is identified by a tag.  It contains an optional series of indices,"] # [doc = " followed by a series of attributes."] # [derive (Debug , Clone)] pub struct AttributesSubsubsection < 'data > { tag : u8 , length : u32 , indices : Bytes < 'data > , data : Bytes < 'data > , }
};
}
