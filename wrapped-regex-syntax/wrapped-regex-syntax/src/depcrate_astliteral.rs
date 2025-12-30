// Generated macro for Literal (struct)
macro_rules! Depcrate_astLiteral {
() => {
// Module: crate::ast
// Provides: {"Literal"}
// Dependencies: {}
# [doc = " A single literal expression."] # [doc = ""] # [doc = " A literal corresponds to a single Unicode scalar value. Literals may be"] # [doc = " represented in their literal form, e.g., `a` or in their escaped form,"] # [doc = " e.g., `\\x61`."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Literal { # [doc = " The span of this literal."] pub span : Span , # [doc = " The kind of this literal."] pub kind : LiteralKind , # [doc = " The Unicode scalar value corresponding to this literal."] pub c : char , }
};
}
