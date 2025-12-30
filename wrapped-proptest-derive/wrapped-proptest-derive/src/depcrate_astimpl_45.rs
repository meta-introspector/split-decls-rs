// Generated macro for impl_45 (impl)
macro_rules! Depcrate_astimpl_45 {
() => {
// Module: crate::ast
// Provides: {"impl_45"}
// Dependencies: {}
impl ToTokens for Strategy { fn to_tokens (& self , tokens : & mut TokenStream) { use self :: Strategy :: * ; match self { Arbitrary (ty , span) => tokens . append_all (quote_spanned ! (* span => <# ty as _proptest :: arbitrary :: Arbitrary >:: Strategy)) , Regex (ty) => quote_append ! (tokens , <# ty as _proptest :: string :: StrategyFromRegex >:: Strategy) , Existential (ty) => quote_append ! (tokens , _proptest :: strategy :: BoxedStrategy <# ty >) , Value (ty) => quote_append ! (tokens , fn () -> # ty) , Map (strats) => { let types = self . types () ; let field_tys = NestedTuple (& types) ; let strats = NestedTuple (& strats) ; quote_append ! (tokens , _proptest :: strategy :: Map < (# strats) , fn (# field_tys) -> Self >) } # [cfg (not (feature = "boxed_union"))] Union (strats) => union_strat_to_tokens (tokens , strats) , # [cfg (feature = "boxed_union")] Union (strats) => union_strat_to_tokens_boxed (tokens , strats) , Filter (strat , ty) => quote_append ! (tokens , _proptest :: strategy :: Filter <# strat , fn (&# ty) -> bool >) , } } }
};
}
