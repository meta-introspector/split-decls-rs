// Generated macro for gcc_features_by_flags (function)
macro_rules! Depcrate_gcc_utilgcc_features_by_flags {
() => {
// Module: crate::gcc_util
// Provides: {"gcc_features_by_flags"}
// Dependencies: {}
fn gcc_features_by_flags (sess : & Session , features : & mut Vec < String >) { target_features :: retpoline_features_by_flags (sess , features) ; }
};
}
