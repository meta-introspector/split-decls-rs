macro_rules! deps {
    () => {
        BytesOrWideString!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > BytesOrWideString < 'a > { # [doc = " Lossy converts to a `Cow<str>`, will allocate if `Bytes` is not valid"] # [doc = " UTF-8 or if `BytesOrWideString` is `Wide`."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] pub fn to_str_lossy (& self) -> Cow < 'a , str > { use self :: BytesOrWideString :: * ; match * self { Bytes (slice) => String :: from_utf8_lossy (slice) , Wide (wide) => Cow :: Owned (String :: from_utf16_lossy (wide)) , } } # [doc = " Provides a `Path` representation of `BytesOrWideString`."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] pub fn into_path_buf (self) -> PathBuf { # [cfg (unix)] { use std :: ffi :: OsStr ; use std :: os :: unix :: ffi :: OsStrExt ; if let BytesOrWideString :: Bytes (slice) = self { return PathBuf :: from (OsStr :: from_bytes (slice)) ; } } # [cfg (windows)] { use std :: ffi :: OsString ; use std :: os :: windows :: ffi :: OsStringExt ; if let BytesOrWideString :: Wide (slice) = self { return PathBuf :: from (OsString :: from_wide (slice)) ; } } if let BytesOrWideString :: Bytes (b) = self { if let Ok (s) = str :: from_utf8 (b) { return PathBuf :: from (s) ; } } unreachable ! () } }
    };
}

impl_32!()