// Generated macro for shallow_find_dir_entries (function)
macro_rules! Depcrate_fsshallow_find_dir_entries {
() => {
// Module: crate::fs
// Provides: {"shallow_find_dir_entries"}
// Dependencies: {}
# [doc = " List directory entries immediately under the given `dir`."] # [track_caller] pub fn shallow_find_dir_entries < P : AsRef < Path > > (dir : P) -> Vec < PathBuf > { let paths = read_dir (dir) ; let mut output = Vec :: new () ; for path in paths { output . push (path . unwrap () . path ()) ; } output }
};
}
