// Generated macro for is_paste_operation (function)
macro_rules! Depcrateis_paste_operation {
() => {
// Module: crate
// Provides: {"is_paste_operation"}
// Dependencies: {}
fn is_paste_operation (input : & TokenStream) -> bool { let mut tokens = input . clone () . into_iter () ; match & tokens . next () { Some (TokenTree :: Punct (punct)) if punct . as_char () == '<' => { } _ => return false , } let mut has_token = false ; loop { match & tokens . next () { Some (TokenTree :: Punct (punct)) if punct . as_char () == '>' => { return has_token && tokens . next () . is_none () ; } Some (_) => has_token = true , None => return false , } } }
};
}
