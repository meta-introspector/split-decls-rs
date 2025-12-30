// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
fn parse (args : TokenStream2 , input : TokenStream2) -> Result < ItemFn > { let function : ItemFn = syn :: parse2 (input) ? ; let _ : Nothing = syn :: parse2 :: < Nothing > (args) ? ; if function . sig . constness . is_some () { return Err (Error :: new (Span :: call_site () , "no_panic attribute on const fn is not supported" ,)) ; } if function . sig . asyncness . is_some () { return Err (Error :: new (Span :: call_site () , "no_panic attribute on async fn is not supported" ,)) ; } Ok (function) }
};
}
