// Generated macro for query_refupdates (function)
macro_rules! Depcrate_config_cache_utilquery_refupdates {
() => {
// Module: crate::config::cache::util
// Provides: {"query_refupdates"}
// Dependencies: {}
pub (crate) fn query_refupdates (config : & gix_config :: File < 'static > , lenient_config : bool ,) -> Result < Option < gix_ref :: store :: WriteReflog > , Error > { let key = "core.logAllRefUpdates" ; Core :: LOG_ALL_REF_UPDATES . try_into_ref_updates (config . boolean (key)) . with_leniency (lenient_config) . map_err (Into :: into) }
};
}
