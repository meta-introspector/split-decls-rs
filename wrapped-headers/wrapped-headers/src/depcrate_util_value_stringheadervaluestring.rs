// Generated macro for HeaderValueString (struct)
macro_rules! Depcrate_util_value_stringHeaderValueString {
() => {
// Module: crate::util::value_string
// Provides: {"HeaderValueString"}
// Dependencies: {}
# [doc = " A value that is both a valid `HeaderValue` and `String`."] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub (crate) struct HeaderValueString { # [doc = " Care must be taken to only set this value when it is also"] # [doc = " a valid `String`, since `as_str` will convert to a `&str`"] # [doc = " in an unchecked manner."] value : HeaderValue , }
};
}
