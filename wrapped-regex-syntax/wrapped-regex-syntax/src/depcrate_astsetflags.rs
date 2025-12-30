// Generated macro for SetFlags (struct)
macro_rules! Depcrate_astSetFlags {
() => {
// Module: crate::ast
// Provides: {"SetFlags"}
// Dependencies: {}
# [doc = " A group of flags that is not applied to a particular regular expression."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct SetFlags { # [doc = " The span of these flags, including the grouping parentheses."] pub span : Span , # [doc = " The actual sequence of flags."] pub flags : Flags , }
};
}
