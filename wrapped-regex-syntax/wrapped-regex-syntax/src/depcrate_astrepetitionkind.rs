// Generated macro for RepetitionKind (enum)
macro_rules! Depcrate_astRepetitionKind {
() => {
// Module: crate::ast
// Provides: {"RepetitionKind"}
// Dependencies: {}
# [doc = " The kind of a repetition operator."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum RepetitionKind { # [doc = " `?`"] ZeroOrOne , # [doc = " `*`"] ZeroOrMore , # [doc = " `+`"] OneOrMore , # [doc = " `{m,n}`"] Range (RepetitionRange) , }
};
}
