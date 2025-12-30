// Generated macro for AttributesSubsubsection (struct)
macro_rules! Depcrate_build_elfAttributesSubsubsection {
() => {
// Module: crate::build::elf
// Provides: {"AttributesSubsubsection"}
// Dependencies: {}
# [doc = " A sub-subsection in an attributes section."] # [derive (Debug , Clone)] pub struct AttributesSubsubsection < 'data > { # [doc = " The sub-subsection tag."] pub tag : AttributeTag , # [doc = " The data containing the attributes."] pub data : Bytes < 'data > , }
};
}
