// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl WideEncoding { # [doc = " Returns the number of code units it takes to encode `text` in this encoding."] pub fn measure (& self , text : & str) -> usize { match self { WideEncoding :: Utf16 => text . encode_utf16 () . count () , WideEncoding :: Utf32 => text . chars () . count () , } } }
};
}
