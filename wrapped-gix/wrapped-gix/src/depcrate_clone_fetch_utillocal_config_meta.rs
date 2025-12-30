// Generated macro for local_config_meta (function)
macro_rules! Depcrate_clone_fetch_utillocal_config_meta {
() => {
// Module: crate::clone::fetch::util
// Provides: {"local_config_meta"}
// Dependencies: {}
fn local_config_meta (repo : & Repository) -> gix_config :: file :: Metadata { let meta = repo . config . resolved . meta () . clone () ; assert_eq ! (meta . source , gix_config :: Source :: Local , "local path is the default for new sections") ; meta }
};
}
