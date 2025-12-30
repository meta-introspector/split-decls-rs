// Generated macro for Repetition (struct)
macro_rules! Depcrate_astRepetition {
() => {
// Module: crate::ast
// Provides: {"Repetition"}
// Dependencies: {}
# [doc = " A repetition operation applied to a regular expression."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Repetition { # [doc = " The span of this operation."] pub span : Span , # [doc = " The actual operation."] pub op : RepetitionOp , # [doc = " Whether this operation was applied greedily or not."] pub greedy : bool , # [doc = " The regular expression under repetition."] pub ast : Box < Ast > , }
};
}
