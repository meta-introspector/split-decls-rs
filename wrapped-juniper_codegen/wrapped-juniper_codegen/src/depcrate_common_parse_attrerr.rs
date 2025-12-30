// Generated macro for err (module)
macro_rules! Depcrate_common_parse_attrerr {
() => {
// Module: crate::common::parse::attr
// Provides: {"err"}
// Dependencies: {}
# [doc = " Common errors of parsing Rust attributes, appeared in this crate."] pub (crate) mod err { use proc_macro2 :: Span ; use syn :: spanned :: Spanned ; # [doc = " Creates \"duplicated argument\" [`syn::Error`] for the given `name` pointing to the given"] # [doc = " `span`."] # [must_use] pub (crate) fn dup_arg < S : AsSpan > (span : S) -> syn :: Error { syn :: Error :: new (span . as_span () , "duplicated attribute argument found") } # [doc = " Creates \"unknown argument\" [`syn::Error`] for the given `name` pointing to the given `span`."] # [must_use] pub (crate) fn unknown_arg < S : AsSpan > (span : S , name : & str) -> syn :: Error { syn :: Error :: new (span . as_span () , format ! ("unknown `{name}` attribute argument") ,) } # [doc = " Helper coercion for [`Span`] and [`Spanned`] types to use in function arguments."] pub (crate) trait AsSpan { # [doc = " Returns the coerced [`Span`]."] # [must_use] fn as_span (& self) -> Span ; } impl AsSpan for Span { # [inline] fn as_span (& self) -> Self { * self } } impl < T : Spanned > AsSpan for & T { # [inline] fn as_span (& self) -> Span { self . span () } } }
};
}
