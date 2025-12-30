// Generated macro for num_files_at_level (function)
macro_rules! Depcrate_propertiesnum_files_at_level {
() => {
// Module: crate::properties
// Provides: {"num_files_at_level"}
// Dependencies: {}
# [doc = " \"rocksdb.num-files-at-level<`N`>\" - returns string containing the number"] # [doc = " of files at level <`N`>, where <`N`> is an ASCII representation of a"] # [doc = " level number (e.g., \"0\")."] pub fn num_files_at_level (level : usize) -> PropertyName { unsafe { level_property ("num-files-at-level" , level) } }
};
}
