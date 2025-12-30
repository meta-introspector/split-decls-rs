// Generated macro for find_name (function)
macro_rules! Depcratefind_name {
() => {
// Module: crate
// Provides: {"find_name"}
// Dependencies: {}
fn find_name (stream : proc_macro2 :: TokenStream) -> Ident { let mut iter = stream . into_iter () ; while let Some (tok) = iter . next () { if let TokenTree :: Ident (ident) = tok { if ident == "fn" { break ; } } } if let Some (TokenTree :: Ident (name)) = iter . next () { name } else { panic ! ("Unable to find function name") } }
};
}
