// Generated macro for AhoCorasick (struct)
macro_rules! Depcrate_util_prefilter_aho_corasickAhoCorasick {
() => {
// Module: crate::util::prefilter::aho_corasick
// Provides: {"AhoCorasick"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct AhoCorasick { # [cfg (not (feature = "perf-literal-multisubstring"))] _unused : () , # [cfg (feature = "perf-literal-multisubstring")] ac : aho_corasick :: AhoCorasick , }
};
}
