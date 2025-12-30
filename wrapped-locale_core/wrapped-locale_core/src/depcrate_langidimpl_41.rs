// Generated macro for impl_41 (impl)
macro_rules! Depcrate_langidimpl_41 {
() => {
// Module: crate::langid
// Provides: {"impl_41"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for LanguageIdentifier { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
