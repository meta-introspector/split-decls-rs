// Generated macro for AttributesSubsection (struct)
macro_rules! Depcrate_build_elfAttributesSubsection {
() => {
// Module: crate::build::elf
// Provides: {"AttributesSubsection"}
// Dependencies: {}
# [doc = " A subsection of an attributes section."] # [derive (Debug , Clone)] pub struct AttributesSubsection < 'data > { # [doc = " The vendor namespace for these attributes."] pub vendor : ByteString < 'data > , # [doc = " The sub-subsections."] pub subsubsections : Vec < AttributesSubsubsection < 'data > > , }
};
}
