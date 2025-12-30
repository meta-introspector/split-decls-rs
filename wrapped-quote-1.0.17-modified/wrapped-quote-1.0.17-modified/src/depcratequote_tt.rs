// Generated macro for quote_tt (macro)
macro_rules! Depcratequote_tt {
() => {
// Module: crate
// Provides: {"quote_tt"}
// Dependencies: {}
# [macro_export] macro_rules ! quote_tt { () => { $ crate :: __private :: TokenStream :: new () } ; ($ ($ tt : tt) *) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; $ crate :: quote_tt_inner ! (_s $ ($ tt) *) ; _s } } ; }
};
}
