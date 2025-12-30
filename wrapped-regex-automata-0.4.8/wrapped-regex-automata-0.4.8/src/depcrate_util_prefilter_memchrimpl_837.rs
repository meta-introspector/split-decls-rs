// Generated macro for impl_837 (impl)
macro_rules! Depcrate_util_prefilter_memchrimpl_837 {
() => {
// Module: crate::util::prefilter::memchr
// Provides: {"impl_837"}
// Dependencies: {}
impl Memchr2 { pub (crate) fn new < B : AsRef < [u8] > > (_kind : MatchKind , needles : & [B] ,) -> Option < Memchr2 > { # [cfg (not (feature = "perf-literal-substring"))] { None } # [cfg (feature = "perf-literal-substring")] { if needles . len () != 2 { return None ; } if ! needles . iter () . all (| n | n . as_ref () . len () == 1) { return None ; } let b1 = needles [0] . as_ref () [0] ; let b2 = needles [1] . as_ref () [0] ; Some (Memchr2 (b1 , b2)) } } }
};
}
