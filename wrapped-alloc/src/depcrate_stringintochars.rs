// Generated macro for IntoChars (struct)
macro_rules! Depcrate_stringIntoChars {
() => {
// Module: crate::string
// Provides: {"IntoChars"}
// Dependencies: {}
# [doc = " An iterator over the [`char`]s of a string."] # [doc = ""] # [doc = " This struct is created by the [`into_chars`] method on [`String`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`char`]: prim@char"] # [doc = " [`into_chars`]: String::into_chars"] # [cfg_attr (not (no_global_oom_handling) , derive (Clone))] # [must_use = "iterators are lazy and do nothing unless consumed"] # [unstable (feature = "string_into_chars" , issue = "133125")] pub struct IntoChars { bytes : vec :: IntoIter < u8 > , }
};
}
