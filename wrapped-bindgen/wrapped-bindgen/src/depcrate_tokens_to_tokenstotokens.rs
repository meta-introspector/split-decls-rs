// Generated macro for ToTokens (trait)
macro_rules! Depcrate_tokens_to_tokensToTokens {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"ToTokens"}
// Dependencies: {}
# [doc = " Types that can be interpolated inside a `quote!` invocation."] # [doc = ""] # [doc = " [`quote!`]: macro.quote.html"] pub trait ToTokens { # [doc = " Write `self` to the given `TokenStream`."] fn to_tokens (& self , tokens : & mut TokenStream) ; # [doc = " Convert `self` directly into a `TokenStream` object."] # [doc = ""] # [doc = " This method is implicitly implemented using `to_tokens`, and acts as a"] # [doc = " convenience method for consumers of the `ToTokens` trait."] fn to_token_stream (& self) -> TokenStream { let mut tokens = TokenStream :: new () ; self . to_tokens (& mut tokens) ; tokens } # [doc = " Convert `self` directly into a `TokenStream` object."] # [doc = ""] # [doc = " This method is implicitly implemented using `to_tokens`, and acts as a"] # [doc = " convenience method for consumers of the `ToTokens` trait."] fn into_token_stream (self) -> TokenStream where Self : Sized , { self . to_token_stream () } }
};
}
