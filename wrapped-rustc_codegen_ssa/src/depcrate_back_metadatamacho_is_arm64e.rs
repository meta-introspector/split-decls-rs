// Generated macro for macho_is_arm64e (function)
macro_rules! Depcrate_back_metadatamacho_is_arm64e {
() => {
// Module: crate::back::metadata
// Provides: {"macho_is_arm64e"}
// Dependencies: {}
# [doc = " Is Apple's CPU subtype `arm64e`s"] fn macho_is_arm64e (target : & Target) -> bool { target . llvm_target . starts_with ("arm64e") }
};
}
