// Generated macro for FallbackOptions (struct)
macro_rules! DepcrateFallbackOptions {
() => {
// Module: crate
// Provides: {"FallbackOptions"}
// Dependencies: {}
# [doc = " Options bag configuring locale inclusion and behavior when runtime fallback is enabled."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub struct FallbackOptions { # [doc = " The aggressiveness of deduplication of data payloads."] pub deduplication_strategy : DeduplicationStrategy , }
};
}
