// Generated macro for impl_317 (impl)
macro_rules! Depcrate_extensions_unicodeimpl_317 {
() => {
// Module: crate::extensions::unicode
// Provides: {"impl_317"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Unicode { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
