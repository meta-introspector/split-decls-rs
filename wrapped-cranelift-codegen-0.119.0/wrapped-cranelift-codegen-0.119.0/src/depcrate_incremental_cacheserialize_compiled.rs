// Generated macro for serialize_compiled (function)
macro_rules! Depcrate_incremental_cacheserialize_compiled {
() => {
// Module: crate::incremental_cache
// Provides: {"serialize_compiled"}
// Dependencies: {}
# [doc = " Given a function that's been successfully compiled, serialize it to a blob that the caller may"] # [doc = " store somewhere for future use by `try_finish_recompile`."] # [doc = ""] # [doc = " As this function requires ownership on the `CompiledCodeStencil`, it gives it back at the end"] # [doc = " of the function call. The value is left untouched."] pub fn serialize_compiled (result : CompiledCodeStencil ,) -> (CompiledCodeStencil , Result < Vec < u8 > , postcard :: Error >) { let cached = CachedFunc { version_marker : VersionMarker , stencil : result , } ; let result = postcard :: to_allocvec (& cached) ; (cached . stencil , result) }
};
}
