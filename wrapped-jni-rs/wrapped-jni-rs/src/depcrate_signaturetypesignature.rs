// Generated macro for TypeSignature (struct)
macro_rules! Depcrate_signatureTypeSignature {
() => {
// Module: crate::signature
// Provides: {"TypeSignature"}
// Dependencies: {}
# [doc = " A method type signature. This is the structure representation of something"] # [doc = " like `(Ljava/lang/String;)Z`. Used by the `call_(object|static)_method`"] # [doc = " functions on jnienv to ensure safety."] # [allow (missing_docs)] # [derive (Eq , PartialEq , Debug , Clone)] pub struct TypeSignature { pub args : Vec < JavaType > , pub ret : ReturnType , }
};
}
