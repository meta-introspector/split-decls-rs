// Generated macro for impl_829 (impl)
macro_rules! Depcrate_util_prefilter_bytesetimpl_829 {
() => {
// Module: crate::util::prefilter::byteset
// Provides: {"impl_829"}
// Dependencies: {}
impl ByteSet { pub (crate) fn new < B : AsRef < [u8] > > (_kind : MatchKind , needles : & [B] ,) -> Option < ByteSet > { # [cfg (not (feature = "perf-literal-multisubstring"))] { None } # [cfg (feature = "perf-literal-multisubstring")] { let mut set = [false ; 256] ; for needle in needles . iter () { let needle = needle . as_ref () ; if needle . len () != 1 { return None ; } set [usize :: from (needle [0])] = true ; } Some (ByteSet (set)) } } }
};
}
