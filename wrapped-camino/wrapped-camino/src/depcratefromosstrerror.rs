// Generated macro for FromOsStrError (struct)
macro_rules! DepcrateFromOsStrError {
() => {
// Module: crate
// Provides: {"FromOsStrError"}
// Dependencies: {}
# [doc = " A possible error value while converting a [`OsStr`] to a [`Utf8Path`]."] # [doc = ""] # [doc = " Produced by the `TryFrom<&OsStr>` implementation for [`&Utf8Path`](Utf8Path)."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::{Utf8Path, FromOsStrError};"] # [doc = " use std::convert::{TryFrom, TryInto};"] # [doc = " use std::ffi::OsStr;"] # [doc = " # #[cfg(unix)]"] # [doc = " use std::os::unix::ffi::OsStrExt;"] # [doc = ""] # [doc = " let unicode_str = OsStr::new(\"/valid/unicode\");"] # [doc = " let utf8_path: &Utf8Path = unicode_str.try_into().expect(\"valid Unicode path succeeded\");"] # [doc = ""] # [doc = " // Paths on Unix can be non-UTF-8."] # [doc = " # #[cfg(unix)]"] # [doc = " let non_unicode_str = OsStr::from_bytes(b\"\\xFF\\xFF\\xFF\");"] # [doc = " # #[cfg(unix)]"] # [doc = " let err: FromOsStrError = <&Utf8Path>::try_from(non_unicode_str)"] # [doc = "     .expect_err(\"non-Unicode path failed\");"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct FromOsStrError (()) ;
};
}
