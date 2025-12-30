// Generated macro for ContentType (enum)
macro_rules! DepcrateContentType {
() => {
// Module: crate
// Provides: {"ContentType"}
// Dependencies: {}
# [doc = " The type of encoding that was detected (for \"text\" data) or `BINARY` for \"binary\" data."] # [allow (non_camel_case_types)] # [derive (Copy , Clone , Debug , PartialEq)] pub enum ContentType { # [doc = " \"binary\" data"] BINARY , # [doc = " UTF-8 encoded \"text\" data"] UTF_8 , # [doc = " UTF-8 encoded \"text\" data with a byte order mark."] UTF_8_BOM , # [doc = " UTF-16 encoded \"text\" data (little endian)"] UTF_16LE , # [doc = " UTF-16 encoded \"text\" data (big endian)"] UTF_16BE , # [doc = " UTF-32 encoded \"text\" data (little endian)"] UTF_32LE , # [doc = " UTF-32 encoded \"text\" data (big endian)"] UTF_32BE , }
};
}
