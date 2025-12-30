// Generated macro for minimal_quote_ts (macro)
macro_rules! Depcrate_quoteminimal_quote_ts {
() => {
// Module: crate::quote
// Provides: {"minimal_quote_ts"}
// Dependencies: {}
macro_rules ! minimal_quote_ts { ((@ $ ($ t : tt) *)) => { $ ($ t) * } ; (::) => { { let mut c = (TokenTree :: from (Punct :: new (':' , Spacing :: Joint)) , TokenTree :: from (Punct :: new (':' , Spacing :: Alone))) ; c . 0 . set_span (Span :: def_site ()) ; c . 1 . set_span (Span :: def_site ()) ; [c . 0 , c . 1] . into_iter () . collect ::< TokenStream > () } } ; (=>) => { { let mut c = (TokenTree :: from (Punct :: new ('=' , Spacing :: Joint)) , TokenTree :: from (Punct :: new ('>' , Spacing :: Alone))) ; c . 0 . set_span (Span :: def_site ()) ; c . 1 . set_span (Span :: def_site ()) ; [c . 0 , c . 1] . into_iter () . collect ::< TokenStream > () } } ; (+=) => { { let mut c = (TokenTree :: from (Punct :: new ('+' , Spacing :: Joint)) , TokenTree :: from (Punct :: new ('=' , Spacing :: Alone))) ; c . 0 . set_span (Span :: def_site ()) ; c . 1 . set_span (Span :: def_site ()) ; [c . 0 , c . 1] . into_iter () . collect ::< TokenStream > () } } ; (!=) => { { let mut c = (TokenTree :: from (Punct :: new ('!' , Spacing :: Joint)) , TokenTree :: from (Punct :: new ('=' , Spacing :: Alone))) ; c . 0 . set_span (Span :: def_site ()) ; c . 1 . set_span (Span :: def_site ()) ; [c . 0 , c . 1] . into_iter () . collect ::< TokenStream > () } } ; ($ t : tt) => { TokenTree :: from (minimal_quote_tt ! ($ t)) } ; }
};
}
