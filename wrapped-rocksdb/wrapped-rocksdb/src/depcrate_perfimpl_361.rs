// Generated macro for impl_361 (impl)
macro_rules! Depcrate_perfimpl_361 {
() => {
// Module: crate::perf
// Provides: {"impl_361"}
// Dependencies: {}
impl Default for PerfContext { fn default () -> Self { let ctx = unsafe { ffi :: rocksdb_perfcontext_create () } ; assert ! (! ctx . is_null () , "Could not create Perf Context") ; Self { inner : ctx } } }
};
}
