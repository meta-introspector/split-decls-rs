// Generated macro for into_iter_1 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsinto_iter_1 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"into_iter_1"}
// Dependencies: {}
macro_rules ! into_iter_1 { ($ module : ident , $ type : ident $ (, $ bound : path) *) => { arbitrary ! ([A : Arbitrary $ (+ $ bound) *] $ module :: IntoIter < A >, SMapped <$ type < A >, Self >, <$ type < A > as Arbitrary >:: Parameters ; args => static_map (any_with ::<$ type < A >> (args) , $ type :: into_iter)) ; lift1 ! (['static + $ ($ bound +) *] $ module :: IntoIter < A >, SizeRange ; base , args => $ module (base , args) . prop_map ($ type :: into_iter)) ; } ; }
};
}
