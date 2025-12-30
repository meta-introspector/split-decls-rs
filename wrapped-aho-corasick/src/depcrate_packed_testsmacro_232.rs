// Generated macro for macro_232 (macro)
macro_rules! Depcrate_packed_testsmacro_232 {
() => {
// Module: crate::packed::tests
// Provides: {"macro_232"}
// Dependencies: {}
testconfig ! (search_teddy_avx2_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_teddy (true) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("avx2") { c . only_teddy_256bit (Some (true)) ; } }) ;
};
}
