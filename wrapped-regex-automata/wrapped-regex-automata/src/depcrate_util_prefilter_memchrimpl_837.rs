// Generated macro for impl_837 (impl)
macro_rules! Depcrate_util_prefilter_memchrimpl_837 {
() => {
// Module: crate::util::prefilter::memchr
// Provides: {"impl_837"}
// Dependencies: {}
impl Memchr { pub (crate) fn new < B : AsRef < [u8] > > (_kind : MatchKind , needles : & [B] ,) -> Option < Memchr > { # [cfg (not (feature = "perf-literal-substring"))] { None } # [cfg (feature = "perf-literal-substring")] { if needles . len () != 1 { return None ; } if needles [0] . as_ref () . len () != 1 { return None ; } Some (Memchr (needles [0] . as_ref () [0])) } } }
};
}
