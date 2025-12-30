// Generated macro for define_emoji_macro (function)
macro_rules! Depcratedefine_emoji_macro {
() => {
// Module: crate
// Provides: {"define_emoji_macro"}
// Dependencies: {}
# [proc_macro] # [decl (fn , name = "define_emoji_macro" , vis = "pub" , hash = "10d99e5c")] pub fn define_emoji_macro (input : TokenStream) -> TokenStream { let input2 : proc_macro2 :: TokenStream = input . into () ; let mut tokens = input2 . into_iter () . peekable () ; if let Some (proc_macro2 :: TokenTree :: Ident (_)) = tokens . peek () { tokens . next () ; } while tokens . next () . is_some () { } TokenStream :: new () }
};
}
