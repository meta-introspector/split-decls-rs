// Generated macro for impl_657 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_657 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_657"}
// Dependencies: {}
impl < 'data > AttributesSubsubsection < 'data > { # [doc = " Return the tag of the attributes sub-subsection."] pub fn tag (& self) -> u8 { self . tag } # [doc = " Return the length of the attributes sub-subsection."] pub fn length (& self) -> u32 { self . length } # [doc = " Return the data containing the indices."] pub fn indices_data (& self) -> & 'data [u8] { self . indices . 0 } # [doc = " Return the indices."] # [doc = ""] # [doc = " This will be section indices if the tag is `Tag_Section`,"] # [doc = " or symbol indices if the tag is `Tag_Symbol`,"] # [doc = " and otherwise it will be empty."] pub fn indices (& self) -> AttributeIndexIterator < 'data > { AttributeIndexIterator { data : self . indices } } # [doc = " Return the data containing the attributes."] pub fn attributes_data (& self) -> & 'data [u8] { self . data . 0 } # [doc = " Return a parser for the data containing the attributes."] pub fn attributes (& self) -> AttributeReader < 'data > { AttributeReader { data : self . data } } }
};
}
