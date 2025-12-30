// Generated macro for impl_63 (impl)
macro_rules! Depcrate_iterators_flat_pairsimpl_63 {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"impl_63"}
// Dependencies: {}
impl < R : RuleType > fmt :: Debug for FlatPairs < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatPairs") . field ("pairs" , & self . clone () . collect :: < Vec < _ > > ()) . finish () } }
};
}
