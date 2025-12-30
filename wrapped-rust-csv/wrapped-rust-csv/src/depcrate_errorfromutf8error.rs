// Generated macro for FromUtf8Error (struct)
macro_rules! Depcrate_errorFromUtf8Error {
() => {
// Module: crate::error
// Provides: {"FromUtf8Error"}
// Dependencies: {}
# [doc = " A UTF-8 validation error during record conversion."] # [doc = ""] # [doc = " This occurs when attempting to convert a `ByteRecord` into a"] # [doc = " `StringRecord`."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct FromUtf8Error { record : ByteRecord , err : Utf8Error , }
};
}
