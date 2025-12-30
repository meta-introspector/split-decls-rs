// Generated macro for impl_249 (impl)
macro_rules! Depcrate_extensions_unicode_attributesimpl_249 {
() => {
// Module: crate::extensions::unicode::attributes
// Provides: {"impl_249"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Attributes { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
