// Generated macro for append_config_to_repo_config (function)
macro_rules! Depcrate_clone_fetch_utilappend_config_to_repo_config {
() => {
// Module: crate::clone::fetch::util
// Provides: {"append_config_to_repo_config"}
// Dependencies: {}
pub fn append_config_to_repo_config (repo : & mut Repository , config : gix_config :: File < 'static >) { let repo_config = gix_features :: threading :: OwnShared :: make_mut (& mut repo . config . resolved) ; repo_config . append (config) ; }
};
}
