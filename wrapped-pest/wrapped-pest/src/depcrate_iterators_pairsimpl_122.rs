// Generated macro for impl_122 (impl)
macro_rules! Depcrate_iterators_pairsimpl_122 {
() => {
// Module: crate::iterators::pairs
// Provides: {"impl_122"}
// Dependencies: {}
impl < R : RuleType > fmt :: Debug for Pairs < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
