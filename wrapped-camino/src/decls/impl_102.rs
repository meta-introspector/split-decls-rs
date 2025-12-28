macro_rules! deps {
    () => {
        FromOsStrError!();
        Utf8Path!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [doc = " Converts an [`OsStr`] to a [`Utf8Path`]."] # [doc = ""] # [doc = " Returns the original [`OsStr`] if it is not valid UTF-8."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8Path;"] # [doc = " use std::convert::TryFrom;"] # [doc = " use std::ffi::OsStr;"] # [doc = " # #[cfg(unix)]"] # [doc = " use std::os::unix::ffi::OsStrExt;"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " # #[cfg(unix)]"] # [doc = " let non_unicode_str = OsStr::from_bytes(b\"\\xFF\\xFF\\xFF\");"] # [doc = " # #[cfg(unix)]"] # [doc = " assert!(<&Utf8Path>::try_from(non_unicode_str).is_err(), \"non-Unicode string path failed\");"] # [doc = " ```"] impl < 'a > TryFrom < & 'a OsStr > for & 'a Utf8Path { type Error = FromOsStrError ; fn try_from (os_str : & 'a OsStr) -> Result < & 'a Utf8Path , Self :: Error > { Utf8Path :: from_os_str (os_str) . ok_or (FromOsStrError (())) } }
    };
}

impl_102!();