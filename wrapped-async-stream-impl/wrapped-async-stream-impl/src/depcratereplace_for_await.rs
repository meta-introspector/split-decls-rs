// Generated macro for replace_for_await (function)
macro_rules! Depcratereplace_for_await {
() => {
// Module: crate
// Provides: {"replace_for_await"}
// Dependencies: {}
# [doc = " Replace `for await` with `#[await] for`, which will be later transformed into a `next` loop."] fn replace_for_await (input : impl IntoIterator < Item = TokenTree >) -> TokenStream2 { let mut input = input . into_iter () . peekable () ; let mut tokens = Vec :: new () ; while let Some (token) = input . next () { match token { TokenTree :: Ident (ident) => { match input . peek () { Some (TokenTree :: Ident (next)) if ident == "for" && next == "await" => { let next_span = next . span () ; let next = syn :: Ident :: new (AWAIT_ATTR_NAME , next_span) ; tokens . extend (quote ! (# [# next])) ; let _ = input . next () ; } _ => { } } tokens . push (ident . into ()) ; } TokenTree :: Group (group) => { let stream = replace_for_await (group . stream ()) ; let mut new_group = Group :: new (group . delimiter () , stream) ; new_group . set_span (group . span ()) ; tokens . push (new_group . into ()) ; } _ => tokens . push (token) , } } tokens . into_iter () . collect () }
};
}
