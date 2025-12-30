// Generated macro for ExtKind (enum)
macro_rules! Depcrate_isa_x64_inst_argsExtKind {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"ExtKind"}
// Dependencies: {}
# [doc = " This defines the ways a value can be extended: either signed- or zero-extension, or none for"] # [doc = " types that are not extended. Contrast with [ExtMode], which defines the widths from and to which"] # [doc = " values can be extended."] # [allow (dead_code)] # [derive (Clone , PartialEq)] pub enum ExtKind { # [doc = " No extension."] None , # [doc = " Sign-extend."] SignExtend , # [doc = " Zero-extend."] ZeroExtend , }
};
}
