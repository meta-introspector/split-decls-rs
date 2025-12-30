// Generated macro for impl_795 (impl)
macro_rules! Depcrate_query_builder_collected_queryimpl_795 {
() => {
// Module: crate::query_builder::collected_query
// Provides: {"impl_795"}
// Dependencies: {}
impl < T > CollectedQuery < T > { # [doc = " Builds a [CollectedQuery] with movable bind data"] pub fn new (sql : String , safe_to_cache_prepared : bool , bind_data : T) -> Self { Self { sql , safe_to_cache_prepared , bind_data , } } }
};
}
