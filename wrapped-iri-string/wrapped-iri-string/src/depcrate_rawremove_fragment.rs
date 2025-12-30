// Generated macro for remove_fragment (function)
macro_rules! Depcrate_rawremove_fragment {
() => {
// Module: crate::raw
// Provides: {"remove_fragment"}
// Dependencies: {}
# [doc = " Removes the fragment part from the string."] # [cfg (feature = "alloc")] # [inline] pub (crate) fn remove_fragment (s : & mut String) { if let Some (colon_pos) = s . find ('#') { s . truncate (colon_pos) ; } }
};
}
