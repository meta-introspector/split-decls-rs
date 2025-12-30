// Generated macro for impl_850 (impl)
macro_rules! Depcrate_util_prefilter_teddyimpl_850 {
() => {
// Module: crate::util::prefilter::teddy
// Provides: {"impl_850"}
// Dependencies: {}
impl Teddy { pub (crate) fn new < B : AsRef < [u8] > > (kind : MatchKind , needles : & [B] ,) -> Option < Teddy > { # [cfg (not (feature = "perf-literal-multisubstring"))] { None } # [cfg (feature = "perf-literal-multisubstring")] { let (packed_match_kind , ac_match_kind) = match kind { MatchKind :: LeftmostFirst | MatchKind :: All => (aho_corasick :: packed :: MatchKind :: LeftmostFirst , aho_corasick :: MatchKind :: LeftmostFirst ,) , } ; let minimum_len = needles . iter () . map (| n | n . as_ref () . len ()) . min () . unwrap_or (0) ; let packed = aho_corasick :: packed :: Config :: new () . match_kind (packed_match_kind) . builder () . extend (needles) . build () ? ; let anchored_ac = aho_corasick :: dfa :: DFA :: builder () . match_kind (ac_match_kind) . start_kind (aho_corasick :: StartKind :: Anchored) . prefilter (false) . build (needles) . ok () ? ; Some (Teddy { searcher : packed , anchored_ac , minimum_len }) } } }
};
}
