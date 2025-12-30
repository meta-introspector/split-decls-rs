// Generated macro for impl_3155 (impl)
macro_rules! Depcrate_incremental_cacheimpl_3155 {
() => {
// Module: crate::incremental_cache
// Provides: {"impl_3155"}
// Dependencies: {}
impl < 'a > CacheKey < 'a > { # [doc = " Creates a new cache store key for a function."] # [doc = ""] # [doc = " This is a bit expensive to compute, so it should be cached and reused as much as possible."] fn new (isa : & dyn TargetIsa , f : & 'a Function) -> Self { CacheKey { stencil : & f . stencil , parameters : CompileParameters :: from_isa (isa) , } } }
};
}
