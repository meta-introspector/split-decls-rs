// Generated macro for Options (struct)
macro_rules! Depcrate_index_traverse_with_lookupOptions {
() => {
// Module: crate::index::traverse::with_lookup
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Traversal options for [`index::File::traverse_with_lookup()`]"] pub struct Options < F > { # [doc = " If `Some`, only use the given number of threads. Otherwise, the number of threads to use will be selected based on"] # [doc = " the number of available logical cores."] pub thread_limit : Option < usize > , # [doc = " The kinds of safety checks to perform."] pub check : index :: traverse :: SafetyCheck , # [doc = " A function to create a pack cache"] pub make_pack_lookup_cache : F , }
};
}
