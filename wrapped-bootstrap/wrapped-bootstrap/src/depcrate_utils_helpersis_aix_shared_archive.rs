// Generated macro for is_aix_shared_archive (function)
macro_rules! Depcrate_utils_helpersis_aix_shared_archive {
() => {
// Module: crate::utils::helpers
// Provides: {"is_aix_shared_archive"}
// Dependencies: {}
fn is_aix_shared_archive (path : & Path) -> bool { let file = match fs :: File :: open (path) { Ok (file) => file , Err (_) => return false , } ; let reader = object :: ReadCache :: new (file) ; let archive = match ArchiveFile :: parse (& reader) { Ok (result) => result , Err (_) => return false , } ; archive . members () . filter_map (Result :: ok) . any (| entry | String :: from_utf8_lossy (entry . name ()) . contains (".so")) }
};
}
