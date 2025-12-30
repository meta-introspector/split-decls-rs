// Generated macro for impl_123 (impl)
macro_rules! Depcrate_iterators_pairsimpl_123 {
() => {
// Module: crate::iterators::pairs
// Provides: {"impl_123"}
// Dependencies: {}
impl < R : RuleType > fmt :: Display for Pairs < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "[{}]" , self . clone () . map (| pair | format ! ("{}" , pair)) . collect ::< Vec < _ >> () . join (", ")) } }
};
}
