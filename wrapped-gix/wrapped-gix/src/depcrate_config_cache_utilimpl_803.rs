// Generated macro for impl_803 (impl)
macro_rules! Depcrate_config_cache_utilimpl_803 {
() => {
// Module: crate::config::cache::util
// Provides: {"impl_803"}
// Dependencies: {}
impl < T , E > ApplyLeniencyDefaultValue < T > for Result < T , E > { fn with_lenient_default_value (self , is_lenient : bool , default : T) -> Self { match self { Ok (v) => Ok (v) , Err (_) if is_lenient => Ok (default) , Err (err) => Err (err) , } } }
};
}
