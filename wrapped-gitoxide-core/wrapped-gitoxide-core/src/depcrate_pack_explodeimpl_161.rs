// Generated macro for impl_161 (impl)
macro_rules! Depcrate_pack_explodeimpl_161 {
() => {
// Module: crate::pack::explode
// Provides: {"impl_161"}
// Dependencies: {}
impl std :: str :: FromStr for SafetyCheck { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "skip-file-checksum" => SafetyCheck :: SkipFileChecksumVerification , "skip-file-and-object-checksum" => SafetyCheck :: SkipFileAndObjectChecksumVerification , "skip-file-and-object-checksum-and-no-abort-on-decode" => { SafetyCheck :: SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError } "all" => SafetyCheck :: All , _ => return Err (format ! ("Unknown value for safety check: '{s}'")) , }) } }
};
}
