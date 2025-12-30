// Generated macro for TypeRef (enum)
macro_rules! Depcrate_dynamic_type_refTypeRef {
() => {
// Module: crate::dynamic::type_ref
// Provides: {"TypeRef"}
// Dependencies: {}
# [doc = " A type reference"] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub enum TypeRef { # [doc = " Named type"] Named (Cow < 'static , str >) , # [doc = " Non-null type"] NonNull (Box < TypeRef >) , # [doc = " List type"] List (Box < TypeRef >) , }
};
}
