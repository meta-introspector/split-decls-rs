// Generated macro for impl_270 (impl)
macro_rules! Depcrate_extensions_unicode_keywordsimpl_270 {
() => {
// Module: crate::extensions::unicode::keywords
// Provides: {"impl_270"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Keywords { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
