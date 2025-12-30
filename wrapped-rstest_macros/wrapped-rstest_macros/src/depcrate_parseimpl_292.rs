// Generated macro for impl_292 (impl)
macro_rules! Depcrate_parseimpl_292 {
() => {
// Module: crate::parse
// Provides: {"impl_292"}
// Dependencies: {}
impl Parse for Fixture { fn parse (input : ParseStream) -> syn :: Result < Self > { let resolve : syn :: Path = input . parse () ? ; if input . peek (Paren) || input . peek (Token ! [as]) { let positional = if input . peek (Paren) { let content ; let _ = syn :: parenthesized ! (content in input) ; content . parse () ? } else { Default :: default () } ; if input . peek (Token ! [as]) { let _ : Token ! [as] = input . parse () ? ; let ident : Ident = input . parse () ? ; Ok (Self :: new (ident . into_pat () , resolve , positional)) } else { let name = resolve . get_ident () . ok_or_else (| | { syn :: Error :: new_spanned (resolve . to_token_stream () , "Should be an ident" . to_string () ,) }) ? ; Ok (Self :: new (name . clone () . into_pat () , name . clone () . into () , positional ,)) } } else { Err (syn :: Error :: new (input . span () , "fixture need arguments or 'as new_name' format" ,)) } } }
};
}
