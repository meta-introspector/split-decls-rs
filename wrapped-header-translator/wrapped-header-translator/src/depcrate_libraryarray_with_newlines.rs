// Generated macro for array_with_newlines (function)
macro_rules! Depcrate_libraryarray_with_newlines {
() => {
// Module: crate::library
// Provides: {"array_with_newlines"}
// Dependencies: {}
fn array_with_newlines (features : impl IntoIterator < Item = String >) -> Item { let mut array : Array = features . into_iter () . collect () ; if 1 < array . len () { for item in array . iter_mut () { item . decor_mut () . set_prefix ("\n    ") ; } array . set_trailing ("\n") ; array . set_trailing_comma (true) ; } value (array) }
};
}
