// Generated macro for impl_180 (impl)
macro_rules! Depcrate_extensions_privateimpl_180 {
() => {
// Module: crate::extensions::private
// Provides: {"impl_180"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Private { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
