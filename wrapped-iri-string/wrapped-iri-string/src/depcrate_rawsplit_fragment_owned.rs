// Generated macro for split_fragment_owned (function)
macro_rules! Depcrate_rawsplit_fragment_owned {
() => {
// Module: crate::raw
// Provides: {"split_fragment_owned"}
// Dependencies: {}
# [doc = " Splits the string into the prefix and the fragment part."] # [doc = ""] # [doc = " A leading `#` character is truncated if the fragment part exists."] # [cfg (feature = "alloc")] pub (crate) fn split_fragment_owned (mut s : String) -> (String , Option < String >) { let prefix_len = match trusted_parser :: split_fragment (& s) { (_ , None) => return (s , None) , (prefix , Some (_fragment)) => prefix . len () , } ; let fragment = s . split_off (prefix_len + 1) ; { let hash = s . pop () ; assert_eq ! (hash , Some ('#')) ; } assert_eq ! (s . len () , prefix_len) ; (s , Some (fragment)) }
};
}
