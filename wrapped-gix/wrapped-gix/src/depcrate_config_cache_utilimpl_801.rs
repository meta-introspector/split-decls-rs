// Generated macro for impl_801 (impl)
macro_rules! Depcrate_config_cache_utilimpl_801 {
() => {
// Module: crate::config::cache::util
// Provides: {"impl_801"}
// Dependencies: {}
impl < T , E > ApplyLeniency for Result < Option < T > , E > { fn with_leniency (self , is_lenient : bool) -> Self { match self { Ok (v) => Ok (v) , Err (_) if is_lenient => Ok (None) , Err (err) => Err (err) , } } }
};
}
