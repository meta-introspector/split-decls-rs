// Generated macro for acceptable_methods (function)
macro_rules! Depcrate_methods_filter_mapacceptable_methods {
() => {
// Module: crate::methods::filter_map
// Provides: {"acceptable_methods"}
// Dependencies: {}
fn acceptable_methods (method : & PathSegment < '_ >) -> bool { matches ! (method . ident . name , sym :: clone | sym :: as_ref | sym :: copied | sym :: cloned | sym :: as_deref | sym :: as_mut | sym :: as_deref_mut | sym :: to_owned) }
};
}
