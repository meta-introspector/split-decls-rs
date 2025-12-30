// Generated macro for JavaType (enum)
macro_rules! Depcrate_signatureJavaType {
() => {
// Module: crate::signature
// Provides: {"JavaType"}
// Dependencies: {}
# [doc = " Enum representing any java type"] # [doc = ""] # [doc = " This intentionally does not keep track of the object class names or details of array elements"] # [doc = " since there would be a cost to tracking those strings and handling variable array dimensions"] # [doc = " while JNI generally only needs to differentiate between primitive types and reference types."] # [doc = ""] # [doc = " In the past this did use to track object names and array details, but it proved to have a"] # [doc = " significant hidden cost that was redundant while those details were never used (at least"] # [doc = " internally)."] # [allow (missing_docs)] # [derive (Eq , PartialEq , Debug , Clone)] pub enum JavaType { Primitive (Primitive) , Object , Array , }
};
}
