// Generated macro for check_repr (function)
macro_rules! Depcratecheck_repr {
() => {
// Module: crate
// Provides: {"check_repr"}
// Dependencies: {}
fn check_repr (input : & DeriveInput) -> Result < () > { let mut has_repr = false ; let mut errors = None ; let mut push_error = | error | match & mut errors { Some (errors) => Error :: combine (errors , error) , None => errors = Some (error) , } ; for attr in & input . attrs { if attr . path () . is_ident ("repr") { if let Err (error) = attr . parse_args_with (| input : ParseStream | { while ! input . is_empty () { let path = input . call (Path :: parse_mod_style) ? ; if path . is_ident ("transparent") || path . is_ident ("C") { has_repr = true ; } else if path . is_ident ("packed") { } else { let meta_item_span = if input . peek (token :: Paren) { let group : TokenTree = input . parse () ? ; quote ! (# path # group) } else if input . peek (Token ! [=]) { let eq_token : Token ! [=] = input . parse () ? ; let value : Expr = input . parse () ? ; quote ! (# path # eq_token # value) } else { quote ! (# path) } ; let msg = if path . is_ident ("align") { "aligned repr on struct that implements RefCast is not supported" } else { "unrecognized repr on struct that implements RefCast" } ; push_error (Error :: new_spanned (meta_item_span , msg)) ; } if ! input . is_empty () { input . parse :: < Token ! [,] > () ? ; } } Ok (()) }) { push_error (error) ; } } } if ! has_repr { let mut requires_repr = Error :: new (Span :: call_site () , "RefCast trait requires #[repr(transparent)]" ,) ; if let Some (errors) = errors { requires_repr . combine (errors) ; } errors = Some (requires_repr) ; } match errors { None => Ok (()) , Some (errors) => Err (errors) , } }
};
}
