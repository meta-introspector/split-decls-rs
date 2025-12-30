// Generated macro for concat (function)
macro_rules! Depcrate_ext_vecconcat {
() => {
// Module: crate::ext_vec
// Provides: {"concat"}
// Dependencies: {}
# [doc = " Concatenate the elements given by the iterator together into a single"] # [doc = " `Vec<u8>`."] # [doc = ""] # [doc = " The elements may be any type that can be cheaply converted into an `&[u8]`."] # [doc = " This includes, but is not limited to, `&str`, `&BStr` and `&[u8]` itself."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr;"] # [doc = ""] # [doc = " let s = bstr::concat(&[\"foo\", \"bar\", \"baz\"]);"] # [doc = " assert_eq!(s, \"foobarbaz\".as_bytes());"] # [doc = " ```"] # [inline] pub fn concat < T , I > (elements : I) -> Vec < u8 > where T : AsRef < [u8] > , I : IntoIterator < Item = T > , { let mut dest = vec ! [] ; for element in elements { dest . push_str (element) ; } dest }
};
}
