// Generated macro for macro_50 (macro)
macro_rules! Depcrate_math_support_feature_detectmacro_50 {
() => {
// Module: crate::math::support::feature_detect
// Provides: {"macro_50"}
// Dependencies: {}
# [cfg (all (target_has_atomic = "ptr" , not (target_has_atomic = "32")))] compile_error ! ("currently all targets that support `AtomicPtr` also support `AtomicU32`") ;
};
}
