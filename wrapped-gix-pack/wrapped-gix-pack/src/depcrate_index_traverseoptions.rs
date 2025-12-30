// Generated macro for Options (struct)
macro_rules! Depcrate_index_traverseOptions {
() => {
// Module: crate::index::traverse
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Traversal options for [`index::File::traverse()`]."] # [derive (Debug , Clone)] pub struct Options < F > { # [doc = " The algorithm to employ."] pub traversal : Algorithm , # [doc = " If `Some`, only use the given number of threads. Otherwise, the number of threads to use will be selected based on"] # [doc = " the number of available logical cores."] pub thread_limit : Option < usize > , # [doc = " The kinds of safety checks to perform."] pub check : SafetyCheck , # [doc = " A function to create a pack cache"] pub make_pack_lookup_cache : F , }
};
}
