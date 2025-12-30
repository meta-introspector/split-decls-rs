// Generated macro for boolean (function)
macro_rules! Depcrate_config_cache_accessboolean {
() => {
// Module: crate::config::cache::access
// Provides: {"boolean"}
// Dependencies: {}
fn boolean (me : & Cache , full_key : & str , key : & 'static config :: tree :: keys :: Boolean , default : bool ,) -> Result < bool , boolean :: Error > { debug_assert_eq ! (full_key , key . logical_name () , "BUG: key name and hardcoded name must match") ; Ok (me . apply_leniency (me . resolved . boolean (full_key) . map (| v | key . enrich_error (v))) ? . unwrap_or (default)) }
};
}
