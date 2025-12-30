// Generated macro for impl_1503 (impl)
macro_rules! Depcrate_stringimpl_1503 {
() => {
// Module: crate::string
// Provides: {"impl_1503"}
// Dependencies: {}
impl IntoChars { # [doc = " Views the underlying data as a subslice of the original data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(string_into_chars)]"] # [doc = ""] # [doc = " let mut chars = String::from(\"abc\").into_chars();"] # [doc = ""] # [doc = " assert_eq!(chars.as_str(), \"abc\");"] # [doc = " chars.next();"] # [doc = " assert_eq!(chars.as_str(), \"bc\");"] # [doc = " chars.next();"] # [doc = " chars.next();"] # [doc = " assert_eq!(chars.as_str(), \"\");"] # [doc = " ```"] # [unstable (feature = "string_into_chars" , issue = "133125")] # [must_use] # [inline] pub fn as_str (& self) -> & str { unsafe { str :: from_utf8_unchecked (self . bytes . as_slice ()) } } # [doc = " Consumes the `IntoChars`, returning the remaining string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(string_into_chars)]"] # [doc = ""] # [doc = " let chars = String::from(\"abc\").into_chars();"] # [doc = " assert_eq!(chars.into_string(), \"abc\");"] # [doc = ""] # [doc = " let mut chars = String::from(\"def\").into_chars();"] # [doc = " chars.next();"] # [doc = " assert_eq!(chars.into_string(), \"ef\");"] # [doc = " ```"] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "string_into_chars" , issue = "133125")] # [inline] pub fn into_string (self) -> String { unsafe { String :: from_utf8_unchecked (self . bytes . collect ()) } } # [inline] fn iter (& self) -> CharIndices < '_ > { self . as_str () . char_indices () } }
};
}
