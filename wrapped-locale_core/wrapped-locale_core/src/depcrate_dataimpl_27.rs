// Generated macro for impl_27 (impl)
macro_rules! Depcrate_dataimpl_27 {
() => {
// Module: crate::data
// Provides: {"impl_27"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromStr for DataLocale { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
