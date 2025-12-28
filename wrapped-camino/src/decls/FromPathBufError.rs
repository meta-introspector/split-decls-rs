macro_rules! deps {
    () => {
        Utf8PathBuf!();
        FromPathError!();
    };
}

macro_rules! FromPathBufError {
    () => {
        deps!();
        # [doc = " A possible error value while converting a [`PathBuf`] to a [`Utf8PathBuf`]."] # [doc = ""] # [doc = " Produced by the [`TryFrom<&PathBuf>`][tryfrom] implementation for [`Utf8PathBuf`]."] # [doc = ""] # [doc = " [tryfrom]: Utf8PathBuf#impl-TryFrom<PathBuf>-for-Utf8PathBuf"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::{Utf8PathBuf, FromPathBufError};"] # [doc = " use std::convert::{TryFrom, TryInto};"] # [doc = " use std::ffi::OsStr;"] # [doc = " # #[cfg(unix)]"] # [doc = " use std::os::unix::ffi::OsStrExt;"] # [doc = " use std::path::PathBuf;"] # [doc = ""] # [doc = " let unicode_path = PathBuf::from(\"/valid/unicode\");"] # [doc = " let utf8_path_buf: Utf8PathBuf = unicode_path.try_into().expect(\"valid Unicode path succeeded\");"] # [doc = ""] # [doc = " // Paths on Unix can be non-UTF-8."] # [doc = " # #[cfg(unix)]"] # [doc = " let non_unicode_str = OsStr::from_bytes(b\"\\xFF\\xFF\\xFF\");"] # [doc = " # #[cfg(unix)]"] # [doc = " let non_unicode_path = PathBuf::from(non_unicode_str);"] # [doc = " # #[cfg(unix)]"] # [doc = " let err: FromPathBufError = Utf8PathBuf::try_from(non_unicode_path.clone())"] # [doc = "     .expect_err(\"non-Unicode path failed\");"] # [doc = " # #[cfg(unix)]"] # [doc = " assert_eq!(err.as_path(), &non_unicode_path);"] # [doc = " # #[cfg(unix)]"] # [doc = " assert_eq!(err.into_path_buf(), non_unicode_path);"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct FromPathBufError { path : PathBuf , error : FromPathError , }
    };
}

FromPathBufError!()