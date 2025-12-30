// Generated macro for impl_232 (impl)
macro_rules! Depcrate_extensions_transformimpl_232 {
() => {
// Module: crate::extensions::transform
// Provides: {"impl_232"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Transform { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
