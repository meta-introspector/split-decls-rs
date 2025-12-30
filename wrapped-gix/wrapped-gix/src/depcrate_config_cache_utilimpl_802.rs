// Generated macro for impl_802 (impl)
macro_rules! Depcrate_config_cache_utilimpl_802 {
() => {
// Module: crate::config::cache::util
// Provides: {"impl_802"}
// Dependencies: {}
impl < T , E > ApplyLeniencyDefault for Result < T , E > where T : Default , { fn with_lenient_default (self , is_lenient : bool) -> Self { match self { Ok (v) => Ok (v) , Err (_) if is_lenient => Ok (T :: default ()) , Err (err) => Err (err) , } } }
};
}
