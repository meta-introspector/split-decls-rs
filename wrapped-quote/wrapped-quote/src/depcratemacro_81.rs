// Generated macro for macro_81 (macro)
macro_rules! Depcratemacro_81 {
() => {
// Module: crate
// Provides: {"macro_81"}
// Dependencies: {}
# [cfg (not (doc))] __quote ! [# [macro_export] macro_rules ! quote { () => { $ crate :: __private :: TokenStream :: new () } ; ($ tt : tt) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; $ crate :: quote_token ! { $ tt _s } _s } } ; (# $ var : ident) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; $ crate :: ToTokens :: to_tokens (&$ var , & mut _s) ; _s } } ; ($ tt1 : tt $ tt2 : tt) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; $ crate :: quote_token ! { $ tt1 _s } $ crate :: quote_token ! { $ tt2 _s } _s } } ; ($ ($ tt : tt) *) => { { let mut _s = $ crate :: __private :: TokenStream :: new () ; $ crate :: quote_each_token ! { _s $ ($ tt) * } _s } } ; }] ;
};
}
