// Generated macro for Utf8LossyChunk (struct)
macro_rules! Depcrate_collections_str_lossyUtf8LossyChunk {
() => {
// Module: crate::collections::str::lossy
// Provides: {"Utf8LossyChunk"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug)] pub struct Utf8LossyChunk < 'a > { # [doc = " Sequence of valid chars."] # [doc = " Can be empty between broken UTF-8 chars."] pub valid : & 'a str , # [doc = " Single broken char, empty if none."] # [doc = " Empty iff iterator item is last."] pub broken : & 'a [u8] , }
};
}
