// Generated macro for impl_95 (impl)
macro_rules! Depcrate_errorimpl_95 {
() => {
// Module: crate::error
// Provides: {"impl_95"}
// Dependencies: {}
impl FromUtf8Error { # [doc = " Create a new FromUtf8Error."] pub (crate) fn new (record : ByteRecord , err : Utf8Error) -> FromUtf8Error { FromUtf8Error { record , err } } # [doc = " Access the underlying `ByteRecord` that failed UTF-8 validation."] pub fn into_byte_record (self) -> ByteRecord { self . record } # [doc = " Access the underlying UTF-8 validation error."] pub fn utf8_error (& self) -> & Utf8Error { & self . err } }
};
}
