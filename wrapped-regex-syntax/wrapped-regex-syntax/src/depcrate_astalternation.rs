// Generated macro for Alternation (struct)
macro_rules! Depcrate_astAlternation {
() => {
// Module: crate::ast
// Provides: {"Alternation"}
// Dependencies: {}
# [doc = " An alternation of regular expressions."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Alternation { # [doc = " The span of this alternation."] pub span : Span , # [doc = " The alternate regular expressions."] pub asts : Vec < Ast > , }
};
}
