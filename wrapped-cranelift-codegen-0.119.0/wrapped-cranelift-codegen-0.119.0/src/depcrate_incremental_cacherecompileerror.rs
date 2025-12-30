// Generated macro for RecompileError (enum)
macro_rules! Depcrate_incremental_cacheRecompileError {
() => {
// Module: crate::incremental_cache
// Provides: {"RecompileError"}
// Dependencies: {}
# [doc = " An error returned when recompiling failed."] # [derive (Debug)] pub enum RecompileError { # [doc = " The version embedded in the cache entry isn't the same as cranelift's current version."] VersionMismatch , # [doc = " An error occurred while deserializing the cache entry."] Deserialize (postcard :: Error) , }
};
}
