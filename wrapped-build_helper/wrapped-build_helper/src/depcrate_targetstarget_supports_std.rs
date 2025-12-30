// Generated macro for target_supports_std (function)
macro_rules! Depcrate_targetstarget_supports_std {
() => {
// Module: crate::targets
// Provides: {"target_supports_std"}
// Dependencies: {}
pub fn target_supports_std (target_tuple : & str) -> bool { ! (target_tuple . contains ("-none") || target_tuple . contains ("nvptx") || target_tuple . contains ("switch")) }
};
}
