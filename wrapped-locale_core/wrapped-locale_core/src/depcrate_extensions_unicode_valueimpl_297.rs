// Generated macro for impl_297 (impl)
macro_rules! Depcrate_extensions_unicode_valueimpl_297 {
() => {
// Module: crate::extensions::unicode::value
// Provides: {"impl_297"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Value { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
