// Generated macro for macro_84 (macro)
macro_rules! Depcratemacro_84 {
() => {
// Module: crate
// Provides: {"macro_84"}
// Dependencies: {}
# [cfg (not (doc))] __quote_spanned ! [# [macro_export] macro_rules ! quote_spanned { ($ span : expr =>) => { { let _ : $ crate :: __private :: Span = $ crate :: __private :: get_span ($ span) . __into_span () ; $ crate :: __private :: TokenStream :: new () } } ; ($ span : expr => $ tt : tt) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; let _span : $ crate :: __private :: Span = $ crate :: __private :: get_span ($ span) . __into_span () ; $ crate :: quote_token_spanned ! { $ tt _s _span } _s } } ; ($ span : expr => # $ var : ident) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; let _ : $ crate :: __private :: Span = $ crate :: __private :: get_span ($ span) . __into_span () ; $ crate :: ToTokens :: to_tokens (&$ var , & mut _s) ; _s } } ; ($ span : expr => $ tt1 : tt $ tt2 : tt) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; let _span : $ crate :: __private :: Span = $ crate :: __private :: get_span ($ span) . __into_span () ; $ crate :: quote_token_spanned ! { $ tt1 _s _span } $ crate :: quote_token_spanned ! { $ tt2 _s _span } _s } } ; ($ span : expr => $ ($ tt : tt) *) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; let _span : $ crate :: __private :: Span = $ crate :: __private :: get_span ($ span) . __into_span () ; $ crate :: quote_each_token_spanned ! { _s _span $ ($ tt) * } _s } } ; }] ;
};
}
