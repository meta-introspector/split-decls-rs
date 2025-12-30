// Generated macro for macro_233 (macro)
macro_rules! Depcrate_packed_testsmacro_233 {
() => {
// Module: crate::packed::tests
// Provides: {"macro_233"}
// Dependencies: {}
testconfig ! (search_teddy_avx2_leftmost_longest , PACKED_LEFTMOST_LONGEST , | c : & mut Config | { c . only_teddy (true) . match_kind (MatchKind :: LeftmostLongest) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("avx2") { c . only_teddy_256bit (Some (true)) ; } }) ;
};
}
