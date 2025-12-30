// Generated macro for custom_strategies (function)
macro_rules! Depcrate_property_test_codegen_arbitrarycustom_strategies {
() => {
// Module: crate::property_test::codegen::arbitrary
// Provides: {"custom_strategies"}
// Dependencies: {}
fn custom_strategies (fn_name : & Ident , args : & [Argument]) -> TokenStream { let arg_strategies : TokenStream = args . iter () . map (| arg | { arg . strategy . as_ref () . map (| s | quote ! { # s , }) . unwrap_or_else (| | { let ty = & arg . pat_ty . ty ; quote_spanned ! { ty . span () => :: proptest :: prelude :: any ::<# ty > () , } } ,) }) . collect () ; let arg_names : TokenStream = args . iter () . enumerate () . map (| (index , _arg) | { let name = nth_field_name (args , index) ; quote ! (# name ,) }) . collect () ; let arg_names = & arg_names ; let strategy_expr = quote ! { use :: proptest :: strategy :: Strategy ; (# arg_strategies) . prop_map (| (# arg_names) | Self { # arg_names }) . boxed () } ; let strategy_type = quote ! { :: proptest :: strategy :: BoxedStrategy < Self > } ; arbitrary_shared (fn_name , strategy_type , strategy_expr) }
};
}
