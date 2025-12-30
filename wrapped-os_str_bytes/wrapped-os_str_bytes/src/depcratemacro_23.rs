// Generated macro for macro_23 (macro)
macro_rules! Depcratemacro_23 {
() => {
// Module: crate
// Provides: {"macro_23"}
// Dependencies: {}
if_checked_conversions ! { # [doc = " The error that occurs when a byte sequence is not representable in the"] # [doc = " platform encoding."] # [doc = ""] # [doc = " [`Result::unwrap`] should almost always be called on results containing"] # [doc = " this error. It should be known whether or not byte sequences are"] # [doc = " properly encoded for the platform, since [the module-level"] # [doc = " documentation][encoding] discourages using encoded bytes in"] # [doc = " interchange. Results are returned primarily to make panicking behavior"] # [doc = " explicit."] # [doc = ""] # [doc = " On Unix, this error is never returned, but [`OsStrExt`] or"] # [doc = " [`OsStringExt`] should be used instead if that needs to be guaranteed."] # [doc = ""] # [doc = " [encoding]: self#encoding-conversions"] # [doc = " [`OsStrExt`]: ::std::os::unix::ffi::OsStrExt"] # [doc = " [`OsStringExt`]: ::std::os::unix::ffi::OsStringExt"] # [doc = " [`Result::unwrap`]: ::std::result::Result::unwrap"] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (os_str_bytes_docs_rs , doc (cfg (feature = "checked_conversions")))] pub struct EncodingError (convert :: EncodingError) ; impl Display for EncodingError { # [inline] fn fmt (& self , f : & mut Formatter <'_ >) -> fmt :: Result { self . 0 . fmt (f) } } impl Error for EncodingError { } }
};
}
