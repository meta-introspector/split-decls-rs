// Generated macro for CacheKey (struct)
macro_rules! Depcrate_incremental_cacheCacheKey {
() => {
// Module: crate::incremental_cache
// Provides: {"CacheKey"}
// Dependencies: {}
# [doc = " Key for caching a single function's compilation."] # [doc = ""] # [doc = " If two functions get the same `CacheKey`, then we can reuse the compiled artifacts, modulo some"] # [doc = " fixups."] # [doc = ""] # [doc = " Note: the key will be invalidated across different versions of cranelift, as the"] # [doc = " `FunctionStencil` contains a `VersionMarker` itself."] # [derive (Hash)] struct CacheKey < 'a > { stencil : & 'a FunctionStencil , parameters : CompileParameters , }
};
}
