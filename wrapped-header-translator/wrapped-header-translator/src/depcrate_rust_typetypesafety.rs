// Generated macro for TypeSafety (struct)
macro_rules! Depcrate_rust_typeTypeSafety {
() => {
// Module: crate::rust_type
// Provides: {"TypeSafety"}
// Dependencies: {}
# [doc = " The safety properties of a type."] # [doc = ""] # [doc = " These depend on which position the type is used in."] # [derive (Debug , PartialEq , Eq , Hash , Clone)] pub struct TypeSafety { # [doc = " The type's safety properties when passed into foreign code."] pub in_argument : SafetyProperty , # [doc = " The type's safety properties when given by foreign code."] pub in_return : SafetyProperty , }
};
}
