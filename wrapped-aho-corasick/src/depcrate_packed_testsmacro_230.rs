// Generated macro for macro_230 (macro)
macro_rules! Depcrate_packed_testsmacro_230 {
() => {
// Module: crate::packed::tests
// Provides: {"macro_230"}
// Dependencies: {}
testconfig ! (search_teddy_ssse3_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_teddy (true) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("ssse3") { c . only_teddy_256bit (Some (false)) ; } }) ;
};
}
