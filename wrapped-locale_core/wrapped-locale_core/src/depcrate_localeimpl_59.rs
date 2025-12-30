// Generated macro for impl_59 (impl)
macro_rules! Depcrate_localeimpl_59 {
() => {
// Module: crate::locale
// Provides: {"impl_59"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for Locale { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
