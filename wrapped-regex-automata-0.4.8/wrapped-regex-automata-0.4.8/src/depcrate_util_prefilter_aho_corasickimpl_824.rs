// Generated macro for impl_824 (impl)
macro_rules! Depcrate_util_prefilter_aho_corasickimpl_824 {
() => {
// Module: crate::util::prefilter::aho_corasick
// Provides: {"impl_824"}
// Dependencies: {}
impl AhoCorasick { pub (crate) fn new < B : AsRef < [u8] > > (kind : MatchKind , needles : & [B] ,) -> Option < AhoCorasick > { # [cfg (not (feature = "perf-literal-multisubstring"))] { None } # [cfg (feature = "perf-literal-multisubstring")] { let ac_match_kind = match kind { MatchKind :: LeftmostFirst | MatchKind :: All => { aho_corasick :: MatchKind :: LeftmostFirst } } ; let ac_kind = if needles . len () <= 500 { aho_corasick :: AhoCorasickKind :: DFA } else { aho_corasick :: AhoCorasickKind :: ContiguousNFA } ; let result = aho_corasick :: AhoCorasick :: builder () . kind (Some (ac_kind)) . match_kind (ac_match_kind) . start_kind (aho_corasick :: StartKind :: Both) . prefilter (false) . build (needles) ; let ac = match result { Ok (ac) => ac , Err (_err) => { debug ! ("aho-corasick prefilter failed to build: {}" , _err) ; return None ; } } ; Some (AhoCorasick { ac }) } } }
};
}
