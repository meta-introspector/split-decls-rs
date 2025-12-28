macro_rules! deps {
    () => {
        ByteVec!();
        Utf8Error!();
    };
}

macro_rules! FromUtf8Error {
    () => {
        deps!();
        # [doc = " An error that may occur when converting a `Vec<u8>` to a `String`."] # [doc = ""] # [doc = " This error includes the original `Vec<u8>` that failed to convert to a"] # [doc = " `String`. This permits callers to recover the allocation used even if it"] # [doc = " it not valid UTF-8."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteVec};"] # [doc = ""] # [doc = " let bytes = Vec::from_slice(b\"foo\\xFFbar\");"] # [doc = " let err = bytes.into_string().unwrap_err();"] # [doc = ""] # [doc = " assert_eq!(err.utf8_error().valid_up_to(), 3);"] # [doc = " assert_eq!(err.utf8_error().error_len(), Some(1));"] # [doc = ""] # [doc = " // At no point in this example is an allocation performed."] # [doc = " let bytes = Vec::from(err.into_vec());"] # [doc = " assert_eq!(bytes, B(b\"foo\\xFFbar\"));"] # [doc = " ```"] # [derive (Debug , Eq , PartialEq)] pub struct FromUtf8Error { original : Vec < u8 > , err : Utf8Error , }
    };
}

FromUtf8Error!()