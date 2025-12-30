// Generated macro for HashValue (struct)
macro_rules! Depcrate_header_mapHashValue {
() => {
// Module: crate::header::map
// Provides: {"HashValue"}
// Dependencies: {}
# [doc = " Hash values are limited to u16 as well. While `fast_hash` and `Hasher`"] # [doc = " return `usize` hash codes, limiting the effective hash code to the lower 16"] # [doc = " bits is fine since we know that the `indices` vector will never grow beyond"] # [doc = " that size."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] struct HashValue (u16) ;
};
}
