// Generated macro for Intrinsic (struct)
macro_rules! Depcrate_common_intrinsicIntrinsic {
() => {
// Module: crate::common::intrinsic
// Provides: {"Intrinsic"}
// Dependencies: {}
# [doc = " An intrinsic"] # [derive (Debug , PartialEq , Clone)] pub struct Intrinsic < T : IntrinsicTypeDefinition > { # [doc = " The function name of this intrinsic."] pub name : String , # [doc = " Any arguments for this intrinsic."] pub arguments : ArgumentList < T > , # [doc = " The return type of this intrinsic."] pub results : T , # [doc = " Any architecture-specific tags."] pub arch_tags : Vec < String > , }
};
}
