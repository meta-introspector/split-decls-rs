// Generated macro for test (module)
macro_rules! Depcrate_optstest {
() => {
// Module: crate::opts
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn smoke () { strict_hash_verification (false) ; } # [test] fn mwindow_size () { unsafe { assert ! (set_mwindow_size (1024) . is_ok ()) ; assert ! (get_mwindow_size () . unwrap () == 1024) ; } } # [test] fn mwindow_mapped_limit () { unsafe { assert ! (set_mwindow_mapped_limit (1024) . is_ok ()) ; assert ! (get_mwindow_mapped_limit () . unwrap () == 1024) ; } } # [test] fn mwindow_file_limit () { unsafe { assert ! (set_mwindow_file_limit (1024) . is_ok ()) ; assert ! (get_mwindow_file_limit () . unwrap () == 1024) ; } } # [test] fn server_connect_timeout () { unsafe { assert ! (set_server_connect_timeout_in_milliseconds (5000) . is_ok ()) ; assert ! (get_server_connect_timeout_in_milliseconds () . unwrap () == 5000) ; } } # [test] fn server_timeout () { unsafe { assert ! (set_server_timeout_in_milliseconds (10_000) . is_ok ()) ; assert ! (get_server_timeout_in_milliseconds () . unwrap () == 10_000) ; } } # [test] fn cache_size () { unsafe { assert ! (set_cache_max_size (20 * 1024 * 1024) . is_ok ()) ; assert ! (get_cached_memory () . is_ok_and (| m | m . 1 == 20 * 1024 * 1024)) ; } } }
};
}
