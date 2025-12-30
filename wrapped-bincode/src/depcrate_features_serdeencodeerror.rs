// Generated macro for EncodeError (enum)
macro_rules! Depcrate_features_serdeEncodeError {
() => {
// Module: crate::features::serde
// Provides: {"EncodeError"}
// Dependencies: {}
# [doc = " A serde-specific error that occurred while encoding."] # [derive (Debug)] # [non_exhaustive] pub enum EncodeError { # [doc = " Serde provided bincode with a sequence without a length, which is not supported in bincode"] SequenceMustHaveLength , # [doc = " [Serializer::collect_str] got called but bincode was unable to allocate memory."] # [cfg (not (feature = "alloc"))] CannotCollectStr , # [doc = " Custom serde error but bincode is unable to allocate a string. Set a breakpoint where this is thrown for more information."] # [cfg (not (feature = "alloc"))] CustomError , }
};
}
