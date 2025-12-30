// Generated macro for impl_161 (impl)
macro_rules! Depcrate_extensions_otherimpl_161 {
() => {
// Module: crate::extensions::other
// Provides: {"impl_161"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Other { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
