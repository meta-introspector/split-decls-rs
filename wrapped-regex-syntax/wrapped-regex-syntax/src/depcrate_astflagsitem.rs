// Generated macro for FlagsItem (struct)
macro_rules! Depcrate_astFlagsItem {
() => {
// Module: crate::ast
// Provides: {"FlagsItem"}
// Dependencies: {}
# [doc = " A single item in a group of flags."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct FlagsItem { # [doc = " The span of this item."] pub span : Span , # [doc = " The kind of this item."] pub kind : FlagsItemKind , }
};
}
