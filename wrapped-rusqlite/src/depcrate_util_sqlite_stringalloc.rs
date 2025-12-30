// Generated macro for alloc (function)
macro_rules! Depcrate_util_sqlite_stringalloc {
() => {
// Module: crate::util::sqlite_string
// Provides: {"alloc"}
// Dependencies: {}
pub (crate) fn alloc (s : & str) -> * mut c_char { SqliteMallocString :: from_str (s) . into_raw () }
};
}
