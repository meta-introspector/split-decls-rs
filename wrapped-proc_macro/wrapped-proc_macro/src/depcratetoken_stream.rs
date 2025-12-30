// Generated macro for token_stream (module)
macro_rules! Depcratetoken_stream {
() => {
// Module: crate
// Provides: {"token_stream"}
// Dependencies: {}
# [doc = " Public implementation details for the `TokenStream` type, such as iterators."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub mod token_stream { use crate :: { Group , Ident , Literal , Punct , TokenStream , TokenTree , bridge } ; # [doc = " An iterator over `TokenStream`'s `TokenTree`s."] # [doc = " The iteration is \"shallow\", e.g., the iterator doesn't recurse into delimited groups,"] # [doc = " and returns whole groups as token trees."] # [derive (Clone)] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub struct IntoIter (std :: vec :: IntoIter < bridge :: TokenTree < bridge :: client :: TokenStream , bridge :: client :: Span , bridge :: client :: Symbol , > , > ,) ; # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl Iterator for IntoIter { type Item = TokenTree ; fn next (& mut self) -> Option < TokenTree > { self . 0 . next () . map (| tree | match tree { bridge :: TokenTree :: Group (tt) => TokenTree :: Group (Group (tt)) , bridge :: TokenTree :: Punct (tt) => TokenTree :: Punct (Punct (tt)) , bridge :: TokenTree :: Ident (tt) => TokenTree :: Ident (Ident (tt)) , bridge :: TokenTree :: Literal (tt) => TokenTree :: Literal (Literal (tt)) , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } fn count (self) -> usize { self . 0 . count () } } # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl IntoIterator for TokenStream { type Item = TokenTree ; type IntoIter = IntoIter ; fn into_iter (self) -> IntoIter { IntoIter (self . 0 . map (| v | v . into_trees ()) . unwrap_or_default () . into_iter ()) } } }
};
}
