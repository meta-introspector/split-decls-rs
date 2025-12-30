// Generated macro for impl_211 (impl)
macro_rules! Depcrate_extensions_transform_valueimpl_211 {
() => {
// Module: crate::extensions::transform::value
// Provides: {"impl_211"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Value { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
