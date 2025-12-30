// Generated macro for impl_48 (impl)
macro_rules! Depcrate_search_attributesimpl_48 {
() => {
// Module: crate::search::attributes
// Provides: {"impl_48"}
// Dependencies: {}
# [doc = " Instantiation and initialization."] impl Search { # [doc = " Create a search instance preloaded with *built-ins* followed by attribute `files` from various global locations."] # [doc = ""] # [doc = " See [`Source`][crate::Source] for a way to obtain these paths."] # [doc = ""] # [doc = " Note that parsing is lenient and errors are logged."] # [doc = ""] # [doc = " * `buf` is used to read `files` from disk which will be ignored if they do not exist."] # [doc = " * `collection` will be updated with information necessary to perform lookups later."] pub fn new_globals (files : impl IntoIterator < Item = impl Into < PathBuf > > , buf : & mut Vec < u8 > , collection : & mut MetadataCollection ,) -> std :: io :: Result < Self > { let mut group = Self :: default () ; group . add_patterns_buffer (b"[attr]binary -diff -merge -text" , "[builtin]" . into () , None , collection , true ,) ; for path in files . into_iter () { group . add_patterns_file (path . into () , true , None , buf , collection , true) ? ; } Ok (group) } }
};
}
