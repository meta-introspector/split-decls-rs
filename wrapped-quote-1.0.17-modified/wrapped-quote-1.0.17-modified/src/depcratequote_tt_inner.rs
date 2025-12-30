// Generated macro for quote_tt_inner (macro)
macro_rules! Depcratequote_tt_inner {
() => {
// Module: crate
// Provides: {"quote_tt_inner"}
// Dependencies: {}
# [macro_export] macro_rules ! quote_tt_inner { ($ tokens : ident # $ var : ident $ ($ tail : tt) *) => { $ crate :: ToTokens :: to_tokens (&$ var , & mut $ tokens) ; $ crate :: quote_tt_inner ! ($ tokens $ ($ tail) *) ; } ; ($ tokens : ident # ($ ($ inner : tt) *) * $ ($ tail : tt) *) => { { use $ crate :: __private :: ext ::*; let has_iter = $ crate :: __private :: ThereIsNoIteratorInRepetition ; $ crate :: pounded_var_names ! (quote_bind_into_iter ! (has_iter) () $ ($ inner) *) ; let _ : $ crate :: __private :: HasIterator = has_iter ; loop { $ crate :: pounded_var_names ! (quote_bind_next_or_break ! () () $ ($ inner) *) ; $ crate :: quote_tt_inner ! ($ tokens $ ($ inner) *) ; if false { break ; } } } ; $ crate :: quote_tt_inner ! ($ tokens $ ($ tail) *) ; } ; ($ tokens : ident # ($ ($ inner : tt) *) $ sep : tt * $ ($ tail : tt) *) => { { use $ crate :: __private :: ext ::*; let mut _i = 0usize ; let has_iter = $ crate :: __private :: ThereIsNoIteratorInRepetition ; $ crate :: pounded_var_names ! (quote_bind_into_iter ! (has_iter) () $ ($ inner) *) ; let _ : $ crate :: __private :: HasIterator = has_iter ; loop { $ crate :: pounded_var_names ! (quote_bind_next_or_break ! () () $ ($ inner) *) ; if _i > 0 { $ crate :: quote_token ! ($ sep $ tokens) ; } _i += 1 ; $ crate :: quote_tt_inner ! ($ tokens $ ($ inner) *) ; if false { break ; } } } ; $ crate :: quote_tt_inner ! ($ tokens $ ($ tail) *) ; } ; ($ tokens : ident $ token : tt $ ($ tail : tt) *) => { $ crate :: quote_token ! ($ token $ tokens) ; $ crate :: quote_tt_inner ! ($ tokens $ ($ tail) *) ; } ; ($ tokens : ident) => { } ; }
};
}
