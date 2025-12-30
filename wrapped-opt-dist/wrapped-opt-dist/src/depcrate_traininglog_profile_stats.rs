// Generated macro for log_profile_stats (function)
macro_rules! Depcrate_traininglog_profile_stats {
() => {
// Module: crate::training
// Provides: {"log_profile_stats"}
// Dependencies: {}
fn log_profile_stats (name : & str , merged_profile : & Utf8Path , profile_root : & Utf8Path ,) -> anyhow :: Result < () > { log :: info ! ("{name} PGO statistics") ; log :: info ! ("{merged_profile}: {}" , humansize :: format_size (std :: fs :: metadata (merged_profile . as_std_path ()) ?. len () , BINARY)) ; log :: info ! ("{profile_root}: {}" , humansize :: format_size (fs_extra :: dir :: get_size (profile_root . as_std_path ()) ?, BINARY)) ; log :: info ! ("Profile file count: {}" , count_files (profile_root) ?) ; Ok (()) }
};
}
