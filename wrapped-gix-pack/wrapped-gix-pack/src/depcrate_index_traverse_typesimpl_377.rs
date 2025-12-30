// Generated macro for impl_377 (impl)
macro_rules! Depcrate_index_traverse_typesimpl_377 {
() => {
// Module: crate::index::traverse::types
// Provides: {"impl_377"}
// Dependencies: {}
impl SafetyCheck { pub (crate) fn file_checksum (& self) -> bool { matches ! (self , SafetyCheck :: All) } pub (crate) fn object_checksum (& self) -> bool { matches ! (self , SafetyCheck :: All | SafetyCheck :: SkipFileChecksumVerification) } pub (crate) fn fatal_decode_error (& self) -> bool { match self { SafetyCheck :: All | SafetyCheck :: SkipFileChecksumVerification | SafetyCheck :: SkipFileAndObjectChecksumVerification => true , SafetyCheck :: SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError => false , } } }
};
}
