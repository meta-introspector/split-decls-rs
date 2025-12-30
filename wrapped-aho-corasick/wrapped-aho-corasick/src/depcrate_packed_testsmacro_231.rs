// Generated macro for macro_231 (macro)
macro_rules! Depcrate_packed_testsmacro_231 {
() => {
// Module: crate::packed::tests
// Provides: {"macro_231"}
// Dependencies: {}
testconfig ! (search_teddy_ssse3_leftmost_longest , PACKED_LEFTMOST_LONGEST , | c : & mut Config | { c . only_teddy (true) . match_kind (MatchKind :: LeftmostLongest) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("ssse3") { c . only_teddy_256bit (Some (false)) ; } }) ;
};
}
