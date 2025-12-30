// Generated macro for impl_3154 (impl)
macro_rules! Depcrate_incremental_cacheimpl_3154 {
() => {
// Module: crate::incremental_cache
// Provides: {"impl_3154"}
// Dependencies: {}
impl CompileParameters { fn from_isa (isa : & dyn TargetIsa) -> Self { Self { isa : isa . name () . to_owned () , triple : isa . triple () . to_string () , flags : isa . flags () . to_string () , isa_flags : isa . isa_flags () . into_iter () . map (| v | v . value_string ()) . collect () , } } }
};
}
