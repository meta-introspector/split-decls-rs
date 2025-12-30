// Generated macro for suffixed_int_literals (macro)
macro_rules! Depcratesuffixed_int_literals {
() => {
// Module: crate
// Provides: {"suffixed_int_literals"}
// Dependencies: {}
macro_rules ! suffixed_int_literals { ($ ($ name : ident => $ kind : ident ,) *) => ($ (# [doc = " Creates a new suffixed integer literal with the specified value."] # [doc = ""] # [doc = " This function will create an integer like `1u32` where the integer"] # [doc = " value specified is the first part of the token and the integral is"] # [doc = " also suffixed at the end. Literals created from negative numbers may"] # [doc = " not survive roundtrips through `TokenStream` or strings and may be"] # [doc = " broken into two tokens (`-` and positive literal)."] # [doc = ""] # [doc = " Literals created through this method have the `Span::call_site()`"] # [doc = " span by default, which can be configured with the `set_span` method"] # [doc = " below."] pub fn $ name (n : $ kind) -> Literal { Literal :: _new (imp :: Literal ::$ name (n)) }) *) }
};
}
