// Generated macro for index_threads (function)
macro_rules! Depcrate_remote_connection_fetch_configindex_threads {
() => {
// Module: crate::remote::connection::fetch::config
// Provides: {"index_threads"}
// Dependencies: {}
pub fn index_threads (repo : & Repository) -> Result < Option < usize > , Error > { Ok (repo . config . resolved . integer_filter (Pack :: THREADS , & mut repo . filter_config_section ()) . map (| threads | Pack :: THREADS . try_into_usize (threads)) . transpose () . with_leniency (repo . options . lenient_config) ?) }
};
}
