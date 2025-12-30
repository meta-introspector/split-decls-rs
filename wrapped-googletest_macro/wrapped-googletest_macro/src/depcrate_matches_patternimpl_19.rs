// Generated macro for impl_19 (impl)
macro_rules! Depcrate_matches_patternimpl_19 {
() => {
// Module: crate::matches_pattern
// Provides: {"impl_19"}
// Dependencies: {}
impl Parse for ParsedMatchPattern { fn parse (input : ParseStream) -> syn :: Result < Self > { let mut struct_name : Vec < TokenTree > = vec ! [] ; let mut group : Option < Group > = None ; input . step (| cursor | { let mut rest = * cursor ; while let Some ((tt , next)) = rest . token_tree () { if let TokenTree :: Group (g) = tt { group = Some (g) ; return Ok ((() , next)) ; } struct_name . push (tt) ; rest = next ; } if matches ! (struct_name . last () , Some (TokenTree :: Punct (p)) if p . as_char () == ',') { struct_name . pop () ; } Ok ((() , rest)) }) ? ; input . parse :: < Option < Token ! [,] > > () ? ; let struct_name = struct_name . into_iter () . collect () ; Ok (ParsedMatchPattern { struct_name , group }) } }
};
}
