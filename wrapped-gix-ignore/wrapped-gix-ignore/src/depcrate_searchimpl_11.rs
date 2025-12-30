// Generated macro for impl_11 (impl)
macro_rules! Depcrate_searchimpl_11 {
() => {
// Module: crate::search
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " Mutation"] impl Search { # [doc = " Add patterns as parsed from `bytes`, providing their `source` path and possibly their `root` path, the path they"] # [doc = " are relative to. This also means that `source` is contained within `root` if `root` is provided."] # [doc = " Use `parse` to control how ignore patterns are parsed."] pub fn add_patterns_buffer (& mut self , bytes : & [u8] , source : impl Into < PathBuf > , root : Option < & Path > , parse : Ignore ,) { self . patterns . push (pattern :: List :: from_bytes (bytes , source . into () , root , parse)) ; } }
};
}
