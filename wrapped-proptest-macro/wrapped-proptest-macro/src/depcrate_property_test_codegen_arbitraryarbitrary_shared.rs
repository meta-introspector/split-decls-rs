// Generated macro for arbitrary_shared (function)
macro_rules! Depcrate_property_test_codegen_arbitraryarbitrary_shared {
() => {
// Module: crate::property_test::codegen::arbitrary
// Provides: {"arbitrary_shared"}
// Dependencies: {}
# [doc = " shared code between both boxed and unboxed paths"] fn arbitrary_shared (fn_name : & Ident , strategy_type : TokenStream , strategy_expr : TokenStream ,) -> TokenStream { let struct_name = struct_name (fn_name) ; quote ! { impl :: proptest :: prelude :: Arbitrary for # struct_name { type Parameters = () ; type Strategy = # strategy_type ; fn arbitrary_with (() : Self :: Parameters) -> Self :: Strategy { # strategy_expr } } } }
};
}
