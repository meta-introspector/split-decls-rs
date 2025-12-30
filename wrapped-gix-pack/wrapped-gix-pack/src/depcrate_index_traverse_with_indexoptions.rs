// Generated macro for Options (struct)
macro_rules! Depcrate_index_traverse_with_indexOptions {
() => {
// Module: crate::index::traverse::with_index
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Traversal options for [`traverse_with_index()`][index::File::traverse_with_index()]"] # [derive (Default)] pub struct Options { # [doc = " If `Some`, only use the given number of threads. Otherwise, the number of threads to use will be selected based on"] # [doc = " the number of available logical cores."] pub thread_limit : Option < usize > , # [doc = " The kinds of safety checks to perform."] pub check : crate :: index :: traverse :: SafetyCheck , }
};
}
