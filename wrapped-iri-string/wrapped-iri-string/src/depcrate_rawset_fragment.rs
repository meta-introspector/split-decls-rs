// Generated macro for set_fragment (function)
macro_rules! Depcrate_rawset_fragment {
() => {
// Module: crate::raw
// Provides: {"set_fragment"}
// Dependencies: {}
# [doc = " Sets the fragment part to the given string."] # [doc = ""] # [doc = " Removes fragment part (and following `#` character) if `None` is given."] # [cfg (feature = "alloc")] pub (crate) fn set_fragment (s : & mut String , fragment : Option < & str >) { remove_fragment (s) ; if let Some (fragment) = fragment { s . reserve (fragment . len () + 1) ; s . push ('#') ; s . push_str (fragment) ; } }
};
}
