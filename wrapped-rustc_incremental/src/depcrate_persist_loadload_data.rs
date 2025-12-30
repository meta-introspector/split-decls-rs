// Generated macro for load_data (function)
macro_rules! Depcrate_persist_loadload_data {
() => {
// Module: crate::persist::load
// Provides: {"load_data"}
// Dependencies: {}
fn load_data (path : & Path , sess : & Session) -> LoadResult < (Mmap , usize) > { match file_format :: read_file (path , sess . opts . unstable_opts . incremental_info , sess . is_nightly_build () , sess . cfg_version ,) { Ok (Some (data_and_pos)) => LoadResult :: Ok { data : data_and_pos } , Ok (None) => { LoadResult :: DataOutOfDate } Err (err) => LoadResult :: LoadDepGraph (path . to_path_buf () , err) , } }
};
}
