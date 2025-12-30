// Generated macro for get_perf_data_filename (function)
macro_rules! Depcrateget_perf_data_filename {
() => {
// Module: crate
// Provides: {"get_perf_data_filename"}
// Dependencies: {}
# [doc = " Traverse a directory looking for `perf.data.<number>` files, and adds one"] # [doc = " to the most recent of those files, returning the new most recent `perf.data`"] # [doc = " file name."] fn get_perf_data_filename (source_path : & Path) -> String { if source_path . join ("perf.data") . exists () { let mut max_number = 0 ; fs :: read_dir (source_path) . unwrap () . filter_map (Result :: ok) . filter (| path | { path . file_name () . as_os_str () . to_string_lossy () . starts_with ("perf.data") }) . for_each (| path | { let file_name = path . file_name () ; let file_name = file_name . as_os_str () . to_str () . unwrap () . split ('.') . next_back () . unwrap () ; if let Ok (parsed_file_name) = file_name . parse :: < usize > () && parsed_file_name >= max_number { max_number = parsed_file_name + 1 ; } }) ; return format ! ("perf.data.{max_number}") ; } String :: from ("perf.data") }
};
}
