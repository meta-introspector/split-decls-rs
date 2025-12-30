// Generated macro for forward_search_bytes (function)
macro_rules! Depcrate_byteset_scalarforward_search_bytes {
() => {
// Module: crate::byteset::scalar
// Provides: {"forward_search_bytes"}
// Dependencies: {}
# [doc = " Safe wrapper around `forward_search`"] # [inline] pub (crate) fn forward_search_bytes < F : Fn (u8) -> bool > (s : & [u8] , confirm : F ,) -> Option < usize > { unsafe { let start = s . as_ptr () ; let end = start . add (s . len ()) ; forward_search (start , end , start , confirm) } }
};
}
