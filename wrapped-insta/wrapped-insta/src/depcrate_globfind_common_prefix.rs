// Generated macro for find_common_prefix (function)
macro_rules! Depcrate_globfind_common_prefix {
() => {
// Module: crate::glob
// Provides: {"find_common_prefix"}
// Dependencies: {}
fn find_common_prefix (sorted_paths : & [PathBuf]) -> Option < & Path > { let first = sorted_paths . first () ? ; let last = sorted_paths . last () ? ; let prefix_len = first . components () . zip (last . components ()) . take_while (| (a , b) | a == b) . count () ; if prefix_len == 0 { None } else { let mut prefix = first . components () ; for _ in 0 .. first . components () . count () - prefix_len { prefix . next_back () ; } Some (prefix . as_path ()) } }
};
}
