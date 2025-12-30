// Generated macro for prefilter (function)
macro_rules! Depcrate_meta_reverse_innerprefilter {
() => {
// Module: crate::meta::reverse_inner
// Provides: {"prefilter"}
// Dependencies: {}
# [doc = " Attempt to extract a prefilter from an HIR expression."] # [doc = ""] # [doc = " We do a little massaging here to do our best that the prefilter we get out"] # [doc = " of this is *probably* fast. Basically, the false positive rate has a much"] # [doc = " higher impact for things like the reverse inner optimization because more"] # [doc = " work needs to potentially be done for each candidate match."] # [doc = ""] # [doc = " Note that this assumes leftmost-first match semantics, so callers must"] # [doc = " not call this otherwise."] fn prefilter (hir : & Hir) -> Option < Prefilter > { let mut extractor = literal :: Extractor :: new () ; extractor . kind (literal :: ExtractKind :: Prefix) ; let mut prefixes = extractor . extract (hir) ; debug ! ("inner prefixes (len={:?}) extracted before optimization: {:?}" , prefixes . len () , prefixes) ; prefixes . make_inexact () ; prefixes . optimize_for_prefix_by_preference () ; debug ! ("inner prefixes (len={:?}) extracted after optimization: {:?}" , prefixes . len () , prefixes) ; prefixes . literals () . and_then (| lits | Prefilter :: new (MatchKind :: LeftmostFirst , lits)) }
};
}
