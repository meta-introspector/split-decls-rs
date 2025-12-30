// Generated macro for no_custom_strategies (function)
macro_rules! Depcrate_property_test_codegen_arbitraryno_custom_strategies {
() => {
// Module: crate::property_test::codegen::arbitrary
// Provides: {"no_custom_strategies"}
// Dependencies: {}
fn no_custom_strategies (fn_name : & Ident , args : & [Argument]) -> TokenStream { let arg_types = args . iter () . map (| arg | { let ty = & arg . pat_ty . ty ; quote ! (# ty ,) }) ; let arg_types = quote ! { # (# arg_types) * } ; let arg_names = args . iter () . enumerate () . map (| (index , _arg) | { let name = nth_field_name (args , index) ; quote ! (# name ,) }) ; let arg_names = quote ! { # (# arg_names) * } ; let strategy_type = quote ! { :: proptest :: strategy :: Map <:: proptest :: arbitrary :: StrategyFor < (# arg_types) >, fn ((# arg_types)) -> Self > } ; let strategy_expr = quote ! { use :: proptest :: strategy :: Strategy ; :: proptest :: prelude :: any ::< (# arg_types) > () . prop_map (| (# arg_names) | Self { # arg_names }) } ; arbitrary_shared (fn_name , strategy_type , strategy_expr) }
};
}
