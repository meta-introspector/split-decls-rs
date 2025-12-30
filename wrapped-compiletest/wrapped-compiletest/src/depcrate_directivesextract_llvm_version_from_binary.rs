// Generated macro for extract_llvm_version_from_binary (function)
macro_rules! Depcrate_directivesextract_llvm_version_from_binary {
() => {
// Module: crate::directives
// Provides: {"extract_llvm_version_from_binary"}
// Dependencies: {}
pub fn extract_llvm_version_from_binary (binary_path : & str) -> Option < Version > { let output = Command :: new (binary_path) . arg ("--version") . output () . ok () ? ; if ! output . status . success () { return None ; } let version = String :: from_utf8 (output . stdout) . ok () ? ; for line in version . lines () { if let Some (version) = line . split ("LLVM version ") . nth (1) { return Some (extract_llvm_version (version)) ; } } None }
};
}
