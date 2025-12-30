// Generated macro for gen_arbitrary_impl (function)
macro_rules! Depcrate_property_test_codegen_arbitrarygen_arbitrary_impl {
() => {
// Module: crate::property_test::codegen::arbitrary
// Provides: {"gen_arbitrary_impl"}
// Dependencies: {}
# [doc = " Generate the arbitrary impl for the struct"] pub (super) fn gen_arbitrary_impl (fn_name : & Ident , args : & [Argument] ,) -> TokenStream { if args . iter () . all (| arg | arg . strategy . is_none ()) { no_custom_strategies (fn_name , args) } else { custom_strategies (fn_name , args) } }
};
}
