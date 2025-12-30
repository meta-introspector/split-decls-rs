// Generated macro for impl_110 (impl)
macro_rules! Depcrate_multi_namedimpl_110 {
() => {
// Module: crate::multi_named
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl FromStr for MultiNamedPlaceholderKeyCow < '_ > { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (MultiNamedPlaceholderKeyCow (Cow :: Owned (String :: from (s)))) } }
};
}
