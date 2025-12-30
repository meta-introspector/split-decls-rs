// Generated macro for Flags (struct)
macro_rules! Depcrate_astFlags {
() => {
// Module: crate::ast
// Provides: {"Flags"}
// Dependencies: {}
# [doc = " A group of flags."] # [doc = ""] # [doc = " This corresponds only to the sequence of flags themselves, e.g., `is-u`."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Flags { # [doc = " The span of this group of flags."] pub span : Span , # [doc = " A sequence of flag items. Each item is either a flag or a negation"] # [doc = " operator."] pub items : Vec < FlagsItem > , }
};
}
