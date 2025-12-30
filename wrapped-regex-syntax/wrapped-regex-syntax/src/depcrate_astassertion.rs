// Generated macro for Assertion (struct)
macro_rules! Depcrate_astAssertion {
() => {
// Module: crate::ast
// Provides: {"Assertion"}
// Dependencies: {}
# [doc = " A single zero-width assertion."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Assertion { # [doc = " The span of this assertion."] pub span : Span , # [doc = " The assertion kind, e.g., `\\b` or `^`."] pub kind : AssertionKind , }
};
}
