// Generated macro for Variant (struct)
macro_rules! Depcrate_astVariant {
() => {
// Module: crate::ast
// Provides: {"Variant"}
// Dependencies: {}
# [doc = " The variant of an enum"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct Variant { # [doc = " The name of this variant"] pub name : Ident , # [doc = " The backing value of this variant"] pub value : u32 , # [doc = " The doc comments on this variant, if any"] pub comments : Vec < String > , }
};
}
