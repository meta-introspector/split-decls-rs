// Generated macro for impl_24 (impl)
macro_rules! Depcrate_search_patternimpl_24 {
() => {
// Module: crate::search::pattern
// Provides: {"impl_24"}
// Dependencies: {}
# [doc = " Utilities"] impl < T > List < T > where T : Pattern , { # [doc = " If this list is anchored to a base path, return `relative_path` as being relative to our base and return"] # [doc = " an updated `basename_pos` as well if it was set."] # [doc = " `case` is respected for the comparison."] # [doc = ""] # [doc = " This is useful to turn repository-relative paths into paths relative to a particular search base."] pub fn strip_base_handle_recompute_basename_pos < 'a > (& self , relative_path : & 'a BStr , basename_pos : Option < usize > , case : Case ,) -> Option < (& 'a BStr , Option < usize >) > { match self . base . as_deref () { Some (base) => strip_base_handle_recompute_basename_pos (base . as_bstr () , relative_path , basename_pos , case) ? , None => (relative_path , basename_pos) , } . into () } }
};
}
