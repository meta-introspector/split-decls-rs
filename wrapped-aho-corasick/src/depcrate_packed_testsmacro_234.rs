// Generated macro for macro_234 (macro)
macro_rules! Depcrate_packed_testsmacro_234 {
() => {
// Module: crate::packed::tests
// Provides: {"macro_234"}
// Dependencies: {}
testconfig ! (search_teddy_fat_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_teddy (true) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("avx2") { c . only_teddy_fat (Some (true)) ; } }) ;
};
}
