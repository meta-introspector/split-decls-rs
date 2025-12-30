// Generated macro for get_cc_search_dirs (function)
macro_rules! Depcrate_core_build_steps_distget_cc_search_dirs {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"get_cc_search_dirs"}
// Dependencies: {}
fn get_cc_search_dirs (target : TargetSelection , builder : & Builder < '_ > ,) -> (Vec < PathBuf > , Vec < PathBuf >) { let mut cmd = command (builder . cc (target)) ; cmd . arg ("-print-search-dirs") ; let gcc_out = cmd . run_capture_stdout (builder) . stdout () ; let mut bin_path : Vec < _ > = env :: split_paths (& env :: var_os ("PATH") . unwrap_or_default ()) . collect () ; let mut lib_path = Vec :: new () ; for line in gcc_out . lines () { let idx = line . find (':') . unwrap () ; let key = & line [.. idx] ; let trim_chars : & [_] = & [' ' , '='] ; let value = env :: split_paths (line [(idx + 1) ..] . trim_start_matches (trim_chars)) ; if key == "programs" { bin_path . extend (value) ; } else if key == "libraries" { lib_path . extend (value) ; } } (bin_path , lib_path) }
};
}
