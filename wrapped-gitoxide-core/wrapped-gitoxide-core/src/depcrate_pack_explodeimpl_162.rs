// Generated macro for impl_162 (impl)
macro_rules! Depcrate_pack_explodeimpl_162 {
() => {
// Module: crate::pack::explode
// Provides: {"impl_162"}
// Dependencies: {}
impl From < SafetyCheck > for pack :: index :: traverse :: SafetyCheck { fn from (v : SafetyCheck) -> Self { use pack :: index :: traverse :: SafetyCheck :: * ; match v { SafetyCheck :: All => All , SafetyCheck :: SkipFileChecksumVerification => SkipFileChecksumVerification , SafetyCheck :: SkipFileAndObjectChecksumVerification => SkipFileAndObjectChecksumVerification , SafetyCheck :: SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError => { SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError } } } }
};
}
