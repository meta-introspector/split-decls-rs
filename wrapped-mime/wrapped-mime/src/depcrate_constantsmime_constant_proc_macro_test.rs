// Generated macro for mime_constant_proc_macro_test (macro)
macro_rules! Depcrate_constantsmime_constant_proc_macro_test {
() => {
// Module: crate::constants
// Provides: {"mime_constant_proc_macro_test"}
// Dependencies: {}
# [cfg (test)] macro_rules ! mime_constant_proc_macro_test { (@ MediaType , $ id : ident , $ src : expr) => (# [cfg (feature = "macro")] { let constant = $ id ; let macroed = media_type ! ($ src) ; assert_eq ! (constant . type_ () , macroed . type_ ()) ; assert_eq ! (constant . subtype () , macroed . subtype ()) ; assert_eq ! (constant . suffix () , macroed . suffix ()) ; assert_ne ! (macroed . mime . private_atom () , 0) ; assert_eq ! (constant . mime . private_atom () , macroed . mime . private_atom ()) ; }) ; (@ MediaRange , $ id : ident , $ src : expr) => () ; }
};
}
