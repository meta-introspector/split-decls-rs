// Generated macro for impl_1 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsimpl_1 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"impl_1"}
// Dependencies: {}
macro_rules ! impl_1 { ($ typ : ident , $ strat : ident , $ ($ bound : path) ,* => $ fun : ident) => { arbitrary ! ([A : Arbitrary $ (+ $ bound) *] $ typ < A >, $ strat < A :: Strategy >, RangedParams1 < A :: Parameters >; args => { let product_unpack ! [range , a] = args ; $ fun (any_with ::< A > (a) , range) }) ; lift1 ! ([$ ($ bound +) *] $ typ < A >, SizeRange ; base , args => $ fun (base , args)) ; } ; }
};
}
