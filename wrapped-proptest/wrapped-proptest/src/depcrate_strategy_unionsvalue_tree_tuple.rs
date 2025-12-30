// Generated macro for value_tree_tuple (macro)
macro_rules! Depcrate_strategy_unionsvalue_tree_tuple {
() => {
// Module: crate::strategy::unions
// Provides: {"value_tree_tuple"}
// Dependencies: {}
macro_rules ! value_tree_tuple { ($ access : ident , $ ($ gen : ident) *) => { impl < A : Strategy , $ ($ gen : Strategy < Value = A :: Value >) ,*> ValueTree for TupleUnionValueTree < (LazyValueTree < A >, $ (Option < LazyValueTree <$ gen >>) ,*) > { lazy_union_value_tree_body ! (A :: Value , $ access) ; } } }
};
}
