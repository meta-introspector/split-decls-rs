// Generated macro for AttributesSection (struct)
macro_rules! Depcrate_build_elfAttributesSection {
() => {
// Module: crate::build::elf
// Provides: {"AttributesSection"}
// Dependencies: {}
# [doc = " The contents of an attributes section."] # [derive (Debug , Default , Clone)] pub struct AttributesSection < 'data > { # [doc = " The subsections."] pub subsections : Vec < AttributesSubsection < 'data > > , }
};
}
