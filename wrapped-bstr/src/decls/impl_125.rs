macro_rules! deps {
    () => {
        ByteVec!();
        Utf8Error!();
        FromUtf8Error!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl FromUtf8Error { # [doc = " Return the original bytes as a slice that failed to convert to a"] # [doc = " `String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteVec};"] # [doc = ""] # [doc = " let bytes = Vec::from_slice(b\"foo\\xFFbar\");"] # [doc = " let err = bytes.into_string().unwrap_err();"] # [doc = ""] # [doc = " // At no point in this example is an allocation performed."] # [doc = " assert_eq!(err.as_bytes(), B(b\"foo\\xFFbar\"));"] # [doc = " ```"] # [inline] pub fn as_bytes (& self) -> & [u8] { & self . original } # [doc = " Consume this error and return the original byte string that failed to"] # [doc = " convert to a `String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteVec};"] # [doc = ""] # [doc = " let bytes = Vec::from_slice(b\"foo\\xFFbar\");"] # [doc = " let err = bytes.into_string().unwrap_err();"] # [doc = " let original = err.into_vec();"] # [doc = ""] # [doc = " // At no point in this example is an allocation performed."] # [doc = " assert_eq!(original, B(b\"foo\\xFFbar\"));"] # [doc = " ```"] # [inline] pub fn into_vec (self) -> Vec < u8 > { self . original } # [doc = " Return the underlying UTF-8 error that occurred. This error provides"] # [doc = " information on the nature and location of the invalid UTF-8 detected."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteVec};"] # [doc = ""] # [doc = " let bytes = Vec::from_slice(b\"foo\\xFFbar\");"] # [doc = " let err = bytes.into_string().unwrap_err();"] # [doc = ""] # [doc = " assert_eq!(err.utf8_error().valid_up_to(), 3);"] # [doc = " assert_eq!(err.utf8_error().error_len(), Some(1));"] # [doc = " ```"] # [inline] pub fn utf8_error (& self) -> & Utf8Error { & self . err } }
    };
}

impl_125!()