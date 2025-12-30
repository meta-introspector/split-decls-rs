// Generated macro for IdentString (struct)
macro_rules! Depcrate_util_ident_stringIdentString {
() => {
// Module: crate::util::ident_string
// Provides: {"IdentString"}
// Dependencies: {}
# [doc = " A wrapper for an `Ident` which also keeps the value as a string."] # [doc = ""] # [doc = " This struct can be used to perform string comparisons and operations."] # [doc = ""] # [doc = " With the optional `serde` feature, this will be serialized as a string and"] # [doc = " supports being deserialized from a string."] # [derive (Clone , PartialOrd , Ord)] pub struct IdentString { ident : Ident , string : String , }
};
}
