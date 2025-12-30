// Generated macro for ParseBufferExt (trait)
macro_rules! Depcrate_common_parseParseBufferExt {
() => {
// Module: crate::common::parse
// Provides: {"ParseBufferExt"}
// Dependencies: {}
# [doc = " Extension of [`ParseBuffer`] providing common function widely used by this crate for parsing."] pub (crate) trait ParseBufferExt { # [doc = " Tries to parse `T` as the next token."] # [doc = ""] # [doc = " Doesn't move [`ParseStream`]'s cursor if there is no `T`."] fn try_parse < T : Default + Parse + Token > (& self) -> syn :: Result < Option < T > > ; # [doc = " Checks whether next token is `T`."] # [doc = ""] # [doc = " Doesn't move [`ParseStream`]'s cursor."] # [must_use] fn is_next < T : Default + Token > (& self) -> bool ; # [doc = " Parses next token as [`syn::Ident`] _allowing_ Rust keywords, while default [`Parse`]"] # [doc = " implementation for [`syn::Ident`] disallows keywords."] # [doc = ""] # [doc = " Always moves [`ParseStream`]'s cursor."] fn parse_any_ident (& self) -> syn :: Result < syn :: Ident > ; # [doc = " Checks whether next token is a wrapper `W` and if yes, then parses the wrapped tokens as `T`"] # [doc = " [`Punctuated`] with `P`. Otherwise, parses just `T`."] # [doc = ""] # [doc = " Always moves [`ParseStream`]'s cursor."] fn parse_maybe_wrapped_and_punctuated < T , W , P > (& self) -> syn :: Result < Punctuated < T , P > > where T : Parse , W : Default + Token + 'static , P : Default + Parse + Token ; }
};
}
