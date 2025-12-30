// Generated macro for impl_1013 (impl)
macro_rules! Depcrate_strategy_unionsimpl_1013 {
() => {
// Module: crate::strategy::unions
// Provides: {"impl_1013"}
// Dependencies: {}
impl < T : Strategy > Clone for UnionValueTree < T > where T :: Tree : Clone , { fn clone (& self) -> Self { Self { options : self . options . clone () , pick : self . pick , min_pick : self . min_pick , prev_pick : self . prev_pick , } } }
};
}
