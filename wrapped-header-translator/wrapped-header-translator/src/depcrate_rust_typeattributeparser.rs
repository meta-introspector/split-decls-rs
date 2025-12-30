// Generated macro for AttributeParser (struct)
macro_rules! Depcrate_rust_typeAttributeParser {
() => {
// Module: crate::rust_type
// Provides: {"AttributeParser"}
// Dependencies: {}
# [doc = " Helper for parsing various attributes."] # [doc = ""] # [doc = " This is _very_ ugly, but required because libclang doesn't expose"] # [doc = " lifetime information."] # [derive (Debug)] struct AttributeParser < 'a , 'b > { _original_name : & 'a str , name : & 'a str , expected_name : & 'b str , }
};
}
