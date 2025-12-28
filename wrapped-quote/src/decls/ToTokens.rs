macro_rules! deps {
    () => {
        TokenStreamExt!();
    };
}

macro_rules! ToTokens {
    () => {
        deps!();
        # [doc = " Types that can be interpolated inside a `quote!` invocation."] pub trait ToTokens { # [doc = " Write `self` to the given `TokenStream`."] # [doc = ""] # [doc = " The token append methods provided by the [`TokenStreamExt`] extension"] # [doc = " trait may be useful for implementing `ToTokens`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Example implementation for a struct representing Rust paths like"] # [doc = " `std::cmp::PartialEq`:"] # [doc = ""] # [doc = " ```"] # [doc = " use proc_macro2::{TokenTree, Spacing, Span, Punct, TokenStream};"] # [doc = " use quote::{TokenStreamExt, ToTokens};"] # [doc = ""] # [doc = " pub struct Path {"] # [doc = "     pub global: bool,"] # [doc = "     pub segments: Vec<PathSegment>,"] # [doc = " }"] # [doc = ""] # [doc = " impl ToTokens for Path {"] # [doc = "     fn to_tokens(&self, tokens: &mut TokenStream) {"] # [doc = "         for (i, segment) in self.segments.iter().enumerate() {"] # [doc = "             if i > 0 || self.global {"] # [doc = "                 // Double colon `::`"] # [doc = "                 tokens.append(Punct::new(':', Spacing::Joint));"] # [doc = "                 tokens.append(Punct::new(':', Spacing::Alone));"] # [doc = "             }"] # [doc = "             segment.to_tokens(tokens);"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " #"] # [doc = " # pub struct PathSegment;"] # [doc = " #"] # [doc = " # impl ToTokens for PathSegment {"] # [doc = " #     fn to_tokens(&self, tokens: &mut TokenStream) {"] # [doc = " #         unimplemented!()"] # [doc = " #     }"] # [doc = " # }"] # [doc = " ```"] fn to_tokens (& self , tokens : & mut TokenStream) ; # [doc = " Convert `self` directly into a `TokenStream` object."] # [doc = ""] # [doc = " This method is implicitly implemented using `to_tokens`, and acts as a"] # [doc = " convenience method for consumers of the `ToTokens` trait."] fn to_token_stream (& self) -> TokenStream { let mut tokens = TokenStream :: new () ; self . to_tokens (& mut tokens) ; tokens } # [doc = " Convert `self` directly into a `TokenStream` object."] # [doc = ""] # [doc = " This method is implicitly implemented using `to_tokens`, and acts as a"] # [doc = " convenience method for consumers of the `ToTokens` trait."] fn into_token_stream (self) -> TokenStream where Self : Sized , { self . to_token_stream () } }
    };
}

ToTokens!()