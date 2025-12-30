// Generated macro for pack_index_version (function)
macro_rules! Depcrate_remote_connection_fetch_configpack_index_version {
() => {
// Module: crate::remote::connection::fetch::config
// Provides: {"pack_index_version"}
// Dependencies: {}
pub fn pack_index_version (repo : & Repository) -> Result < gix_pack :: index :: Version , Error > { Ok (repo . config . resolved . integer (Pack :: INDEX_VERSION) . map (| value | Pack :: INDEX_VERSION . try_into_index_version (value)) . transpose () . with_leniency (repo . options . lenient_config) ? . unwrap_or (gix_pack :: index :: Version :: V2)) }
};
}
