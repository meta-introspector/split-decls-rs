// Generated macro for macro_331 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_331 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_331"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary] Bound < A >, TupleUnion < (WA < SFnPtrMap < Arc < A :: Strategy >, Self >>, WA < SFnPtrMap < Arc < A :: Strategy >, Self >>, WA < LazyJustFn < Self >>) >, A :: Parameters ; args => { let base = Arc :: new (any_with ::< A > (args)) ; prop_oneof ! [2 => static_map (base . clone () , Bound :: Included) , 2 => static_map (base , Bound :: Excluded) , 1 => LazyJust :: new (|| Bound :: Unbounded) ,] }) ;
};
}
