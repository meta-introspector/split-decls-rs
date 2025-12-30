// Generated macro for span_eq (macro)
macro_rules! Depcrate_spanspan_eq {
() => {
// Module: crate::span
// Provides: {"span_eq"}
// Dependencies: {}
# [doc = " A macro helper, only used in tests, for comparing spans for equality."] # [cfg (test)] macro_rules ! span_eq { ($ span1 : expr , $ span2 : expr $ (,) ?) => { { assert_eq ! ($ span1 . fieldwise () , $ span2 . fieldwise ()) ; } } ; ($ span1 : expr , $ span2 : expr , $ ($ tt : tt) *) => { { assert_eq ! ($ span1 . fieldwise () , $ span2 . fieldwise () , $ ($ tt) *) ; } } ; }
};
}
