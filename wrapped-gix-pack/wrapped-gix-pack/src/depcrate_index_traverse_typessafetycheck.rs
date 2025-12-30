// Generated macro for SafetyCheck (enum)
macro_rules! Depcrate_index_traverse_typesSafetyCheck {
() => {
// Module: crate::index::traverse::types
// Provides: {"SafetyCheck"}
// Dependencies: {}
# [doc = " The ways to validate decoded objects before passing them to the processor."] # [derive (Default , Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum SafetyCheck { # [doc = " Don't verify the validity of the checksums stored in the index and pack file"] SkipFileChecksumVerification , # [doc = " All of the above, and also don't perform any object checksum verification"] SkipFileAndObjectChecksumVerification , # [doc = " All of the above, and only log object decode errors."] # [doc = ""] # [doc = " Useful if there is a damaged pack and you would like to traverse as many objects as possible."] SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError , # [doc = " Perform all available safety checks before operating on the pack and"] # [doc = " abort if any of them fails"] # [default] All , }
};
}
