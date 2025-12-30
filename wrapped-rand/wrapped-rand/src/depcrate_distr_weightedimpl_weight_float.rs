// Generated macro for impl_weight_float (macro)
macro_rules! Depcrate_distr_weightedimpl_weight_float {
() => {
// Module: crate::distr::weighted
// Provides: {"impl_weight_float"}
// Dependencies: {}
macro_rules ! impl_weight_float { ($ t : ty) => { impl Weight for $ t { const ZERO : Self = 0.0 ; fn checked_add_assign (& mut self , v : & Self) -> Result < () , () > { * self += * v ; Ok (()) } } } ; }
};
}
