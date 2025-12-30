// Generated macro for is_up_to_date (function)
macro_rules! Depcrateis_up_to_date {
() => {
// Module: crate
// Provides: {"is_up_to_date"}
// Dependencies: {}
# [doc = " Checks whether a particular test/revision is \"up-to-date\", meaning that no"] # [doc = " relevant files/settings have changed since the last time the test succeeded."] # [doc = ""] # [doc = " (This is not very reliable in some circumstances, so the `--force-rerun`"] # [doc = " flag can be used to ignore up-to-date checking and always re-run tests.)"] fn is_up_to_date (cx : & TestCollectorCx , testpaths : & TestPaths , props : & EarlyProps , revision : Option < & str > ,) -> bool { let stamp_file_path = stamp_file_path (& cx . config , testpaths , revision) ; let contents = match fs :: read_to_string (& stamp_file_path) { Ok (f) => f , Err (ref e) if e . kind () == ErrorKind :: InvalidData => panic ! ("Can't read stamp contents") , Err (_) => return false , } ; let expected_hash = runtest :: compute_stamp_hash (& cx . config) ; if contents != expected_hash { return false ; } let mut inputs_stamp = cx . common_inputs_stamp . clone () ; for path in files_related_to_test (& cx . config , testpaths , props , revision) { inputs_stamp . add_path (& path) ; } inputs_stamp < Stamp :: from_path (& stamp_file_path) }
};
}
