// Generated macro for read_dir_entries (function)
macro_rules! Depcrate_fsread_dir_entries {
() => {
// Module: crate::fs
// Provides: {"read_dir_entries"}
// Dependencies: {}
# [doc = " Helper for reading entries in a given directory."] pub fn read_dir_entries < P : AsRef < Path > , F : FnMut (& Path) > (dir : P , mut callback : F) { for entry in read_dir (dir) { callback (& entry . unwrap () . path ()) ; } }
};
}
