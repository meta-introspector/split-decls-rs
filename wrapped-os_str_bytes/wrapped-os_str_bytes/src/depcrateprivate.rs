// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
mod private { use std :: ffi :: OsStr ; use std :: ffi :: OsString ; use std :: path :: Path ; use std :: path :: PathBuf ; if_raw_str ! { use std :: borrow :: Cow ; use super :: RawOsStr ; } pub trait Sealed { } impl Sealed for char { } impl Sealed for OsStr { } impl Sealed for OsString { } impl Sealed for Path { } impl Sealed for PathBuf { } impl Sealed for & str { } impl Sealed for & String { } if_raw_str ! { impl Sealed for Cow <'_ , RawOsStr > { } } }
};
}
