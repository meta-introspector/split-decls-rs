// Generated macro for unsuffixed_int_literals (macro)
macro_rules! Depcrateunsuffixed_int_literals {
() => {
// Module: crate
// Provides: {"unsuffixed_int_literals"}
// Dependencies: {}
macro_rules ! unsuffixed_int_literals { ($ ($ name : ident => $ kind : ident ,) *) => ($ (# [doc = " Creates a new unsuffixed integer literal with the specified value."] # [doc = ""] # [doc = " This function will create an integer like `1` where the integer"] # [doc = " value specified is the first part of the token. No suffix is"] # [doc = " specified on this token, meaning that invocations like"] # [doc = " `Literal::i8_unsuffixed(1)` are equivalent to"] # [doc = " `Literal::u32_unsuffixed(1)`."] # [doc = " Literals created from negative numbers might not survive rountrips through"] # [doc = " `TokenStream` or strings and may be broken into two tokens (`-` and positive literal)."] # [doc = ""] # [doc = " Literals created through this method have the `Span::call_site()`"] # [doc = " span by default, which can be configured with the `set_span` method"] # [doc = " below."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub fn $ name (n : $ kind) -> Literal { Literal (bridge :: Literal { kind : bridge :: LitKind :: Integer , symbol : bridge :: client :: Symbol :: new (& n . to_string ()) , suffix : None , span : Span :: call_site () . 0 , }) }) *) }
};
}
