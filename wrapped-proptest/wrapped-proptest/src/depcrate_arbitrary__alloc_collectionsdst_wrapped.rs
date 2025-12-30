// Generated macro for dst_wrapped (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsdst_wrapped {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"dst_wrapped"}
// Dependencies: {}
macro_rules ! dst_wrapped { ($ ($ w : ident) ,*) => { $ (arbitrary ! ([A : Arbitrary] $ w < [A] >, MapInto < StrategyFor < Vec < A >>, Self >, < Vec < A > as Arbitrary >:: Parameters ; a => any_with ::< Vec < A >> (a) . prop_map_into ()) ;) * } ; }
};
}
