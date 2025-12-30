// Generated macro for suffixed_int_literals (macro)
macro_rules! Depcratesuffixed_int_literals {
() => {
// Module: crate
// Provides: {"suffixed_int_literals"}
// Dependencies: {}
macro_rules ! suffixed_int_literals { ($ ($ name : ident => $ kind : ident ,) *) => ($ (# [doc = " Creates a new suffixed integer literal with the specified value."] # [doc = ""] # [doc = " This function will create an integer like `1u32` where the integer"] # [doc = " value specified is the first part of the token and the integral is"] # [doc = " also suffixed at the end."] # [doc = " Literals created from negative numbers might not survive round-trips through"] # [doc = " `TokenStream` or strings and may be broken into two tokens (`-` and positive literal)."] # [doc = ""] # [doc = " Literals created through this method have the `Span::call_site()`"] # [doc = " span by default, which can be configured with the `set_span` method"] # [doc = " below."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub fn $ name (n : $ kind) -> Literal { Literal (bridge :: Literal { kind : bridge :: LitKind :: Integer , symbol : bridge :: client :: Symbol :: new (& n . to_string ()) , suffix : Some (bridge :: client :: Symbol :: new (stringify ! ($ kind))) , span : Span :: call_site () . 0 , }) }) *) }
};
}
