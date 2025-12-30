// Generated macro for UTF_8 (const)
macro_rules! Depcrate_valueUTF_8 {
() => {
// Module: crate::value
// Provides: {"UTF_8"}
// Dependencies: {}
# [doc = " a `Value` usable for a charset parameter."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " let mime = mime::TEXT_PLAIN_UTF_8;"] # [doc = " assert_eq!(mime.param(mime::CHARSET), Some(mime::UTF_8));"] # [doc = " ```"] pub const UTF_8 : Value = Value { source : "utf-8" , ascii_case_insensitive : true , } ;
};
}
