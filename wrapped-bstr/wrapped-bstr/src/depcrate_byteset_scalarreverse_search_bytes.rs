// Generated macro for reverse_search_bytes (function)
macro_rules! Depcrate_byteset_scalarreverse_search_bytes {
() => {
// Module: crate::byteset::scalar
// Provides: {"reverse_search_bytes"}
// Dependencies: {}
# [doc = " Safe wrapper around `reverse_search`"] # [inline] pub (crate) fn reverse_search_bytes < F : Fn (u8) -> bool > (s : & [u8] , confirm : F ,) -> Option < usize > { unsafe { let start = s . as_ptr () ; let end = start . add (s . len ()) ; reverse_search (start , end , end , confirm) } }
};
}
