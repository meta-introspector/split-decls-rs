// Generated macro for cpp_like_debuginfo (function)
macro_rules! Depcrate_debuginfo_type_namescpp_like_debuginfo {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"cpp_like_debuginfo"}
// Dependencies: {}
# [doc = " Check if we should generate C++ like names and debug information."] pub fn cpp_like_debuginfo (tcx : TyCtxt < '_ >) -> bool { tcx . sess . target . is_like_msvc }
};
}
