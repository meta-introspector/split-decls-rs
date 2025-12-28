macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! Utf8Error {
    () => {
        deps!();
        # [doc = " An error that occurs when UTF-8 decoding fails."] # [doc = ""] # [doc = " This error occurs when attempting to convert a non-UTF-8 byte"] # [doc = " string to a Rust string that must be valid UTF-8. For example,"] # [doc = " [`to_str`](trait.ByteSlice.html#method.to_str) is one such method."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows what happens when a given byte sequence is invalid,"] # [doc = " but ends with a sequence that is a possible prefix of valid UTF-8."] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::{B, ByteSlice};"] # [doc = ""] # [doc = " let s = B(b\"foobar\\xF1\\x80\\x80\");"] # [doc = " let err = s.to_str().unwrap_err();"] # [doc = " assert_eq!(err.valid_up_to(), 6);"] # [doc = " assert_eq!(err.error_len(), None);"] # [doc = " ```"] # [doc = ""] # [doc = " This example shows what happens when a given byte sequence contains"] # [doc = " invalid UTF-8."] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let s = b\"foobar\\xF1\\x80\\x80quux\";"] # [doc = " let err = s.to_str().unwrap_err();"] # [doc = " assert_eq!(err.valid_up_to(), 6);"] # [doc = " // The error length reports the maximum number of bytes that correspond to"] # [doc = " // a valid prefix of a UTF-8 encoded codepoint."] # [doc = " assert_eq!(err.error_len(), Some(3));"] # [doc = ""] # [doc = " // In contrast to the above which contains a single invalid prefix,"] # [doc = " // consider the case of multiple individual bytes that are never valid"] # [doc = " // prefixes. Note how the value of error_len changes!"] # [doc = " let s = b\"foobar\\xFF\\xFFquux\";"] # [doc = " let err = s.to_str().unwrap_err();"] # [doc = " assert_eq!(err.valid_up_to(), 6);"] # [doc = " assert_eq!(err.error_len(), Some(1));"] # [doc = ""] # [doc = " // The fact that it's an invalid prefix does not change error_len even"] # [doc = " // when it immediately precedes the end of the string."] # [doc = " let s = b\"foobar\\xFF\";"] # [doc = " let err = s.to_str().unwrap_err();"] # [doc = " assert_eq!(err.valid_up_to(), 6);"] # [doc = " assert_eq!(err.error_len(), Some(1));"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Utf8Error { valid_up_to : usize , error_len : Option < usize > , }
    };
}

Utf8Error!()