// Generated macro for is_single_interpolation_group (function)
macro_rules! Depcrateis_single_interpolation_group {
() => {
// Module: crate
// Provides: {"is_single_interpolation_group"}
// Dependencies: {}
fn is_single_interpolation_group (input : & TokenStream) -> bool { # [derive (PartialEq)] enum State { Init , Ident , Literal , Apostrophe , Lifetime , Colon1 , Colon2 , } let mut state = State :: Init ; for tt in input . clone () { state = match (state , & tt) { (State :: Init , TokenTree :: Ident (_)) => State :: Ident , (State :: Init , TokenTree :: Literal (_)) => State :: Literal , (State :: Init , TokenTree :: Punct (punct)) if punct . as_char () == '\'' => State :: Apostrophe , (State :: Apostrophe , TokenTree :: Ident (_)) => State :: Lifetime , (State :: Ident , TokenTree :: Punct (punct)) if punct . as_char () == ':' && punct . spacing () == Spacing :: Joint => { State :: Colon1 } (State :: Colon1 , TokenTree :: Punct (punct)) if punct . as_char () == ':' && punct . spacing () == Spacing :: Alone => { State :: Colon2 } (State :: Colon2 , TokenTree :: Ident (_)) => State :: Ident , _ => return false , } ; } state == State :: Ident || state == State :: Literal || state == State :: Lifetime }
};
}
