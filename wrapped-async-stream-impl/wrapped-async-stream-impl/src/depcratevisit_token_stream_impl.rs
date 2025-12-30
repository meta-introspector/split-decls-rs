// Generated macro for visit_token_stream_impl (function)
macro_rules! Depcratevisit_token_stream_impl {
() => {
// Module: crate
// Provides: {"visit_token_stream_impl"}
// Dependencies: {}
fn visit_token_stream_impl (visitor : & mut Scrub < '_ > , tokens : TokenStream2 , modified : & mut bool , out : & mut TokenStream2 ,) { use quote :: ToTokens ; use quote :: TokenStreamExt ; let mut tokens = tokens . into_iter () . peekable () ; while let Some (tt) = tokens . next () { match tt { TokenTree :: Ident (i) if i == "yield" => { let stream = std :: iter :: once (TokenTree :: Ident (i)) . chain (tokens) . collect () ; match syn :: parse2 (stream) { Ok (Partial (yield_expr , rest)) => { let mut expr = syn :: Expr :: Yield (yield_expr) ; visitor . visit_expr_mut (& mut expr) ; expr . to_tokens (out) ; * modified = true ; tokens = rest . into_iter () . peekable () ; } Err (e) => { out . append_all (e . to_compile_error () . into_iter ()) ; * modified = true ; return ; } } } TokenTree :: Ident (i) if i == "stream" || i == "try_stream" => { out . append (TokenTree :: Ident (i)) ; match tokens . peek () { Some (TokenTree :: Punct (p)) if p . as_char () == '!' => { out . extend (tokens . next ()) ; if let Some (TokenTree :: Group (_)) = tokens . peek () { out . extend (tokens . next ()) ; } } _ => { } } } TokenTree :: Group (group) => { let mut content = group . stream () ; * modified |= visitor . visit_token_stream (& mut content) ; let mut new = Group :: new (group . delimiter () , content) ; new . set_span (group . span ()) ; out . append (new) ; } other => out . append (other) , } } }
};
}
