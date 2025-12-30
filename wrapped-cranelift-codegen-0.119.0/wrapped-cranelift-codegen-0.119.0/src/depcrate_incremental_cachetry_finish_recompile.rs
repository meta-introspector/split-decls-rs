// Generated macro for try_finish_recompile (function)
macro_rules! Depcrate_incremental_cachetry_finish_recompile {
() => {
// Module: crate::incremental_cache
// Provides: {"try_finish_recompile"}
// Dependencies: {}
# [doc = " Given a function that's been precompiled and its entry in the caching storage, try to shortcut"] # [doc = " compilation of the given function."] # [doc = ""] # [doc = " Precondition: the bytes must have retrieved from a cache store entry which hash value"] # [doc = " is strictly the same as the `Function`'s computed hash retrieved from `compute_cache_key`."] pub fn try_finish_recompile (func : & Function , bytes : & [u8]) -> Result < CompiledCode , RecompileError > { match postcard :: from_bytes :: < CachedFunc > (bytes) { Ok (result) => { if result . version_marker != func . stencil . version_marker { Err (RecompileError :: VersionMismatch) } else { Ok (result . stencil . apply_params (& func . params)) } } Err (err) => Err (RecompileError :: Deserialize (err)) , } }
};
}
