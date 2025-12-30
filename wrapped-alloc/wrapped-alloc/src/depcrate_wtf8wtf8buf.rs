// Generated macro for Wtf8Buf (struct)
macro_rules! Depcrate_wtf8Wtf8Buf {
() => {
// Module: crate::wtf8
// Provides: {"Wtf8Buf"}
// Dependencies: {}
# [doc = " An owned, growable string of well-formed WTF-8 data."] # [doc = ""] # [doc = " Similar to `String`, but can additionally contain surrogate code points"] # [doc = " if they’re not in a surrogate pair."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone)] # [doc (hidden)] pub struct Wtf8Buf { bytes : Vec < u8 > , # [doc = " Do we know that `bytes` holds a valid UTF-8 encoding? We can easily"] # [doc = " know this if we're constructed from a `String` or `&str`."] # [doc = ""] # [doc = " It is possible for `bytes` to have valid UTF-8 without this being"] # [doc = " set, such as when we're concatenating `&Wtf8`'s and surrogates become"] # [doc = " paired, as we don't bother to rescan the entire string."] is_known_utf8 : bool , }
};
}
