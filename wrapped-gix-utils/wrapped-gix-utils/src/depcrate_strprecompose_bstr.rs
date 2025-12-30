// Generated macro for precompose_bstr (function)
macro_rules! Depcrate_strprecompose_bstr {
() => {
// Module: crate::str
// Provides: {"precompose_bstr"}
// Dependencies: {}
# [doc = " Return the precomposed version of `s`, or `s` itself if it contained illformed unicode,"] # [doc = " or if the unicode version didn't contains decomposed unicode."] # [doc = " Otherwise, similar to [`precompose()`]"] # [cfg (feature = "bstr")] pub fn precompose_bstr (s : Cow < '_ , bstr :: BStr >) -> Cow < '_ , bstr :: BStr > { use bstr :: ByteSlice ; match s . to_str () . ok () { None => s , Some (maybe_decomposed) => match precompose (maybe_decomposed . into ()) { Cow :: Borrowed (_) => s , Cow :: Owned (precomposed) => Cow :: Owned (precomposed . into ()) , } , } }
};
}
