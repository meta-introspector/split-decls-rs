// Generated macro for impl_weight_int (macro)
macro_rules! Depcrate_distr_weightedimpl_weight_int {
() => {
// Module: crate::distr::weighted
// Provides: {"impl_weight_int"}
// Dependencies: {}
macro_rules ! impl_weight_int { ($ t : ty) => { impl Weight for $ t { const ZERO : Self = 0 ; fn checked_add_assign (& mut self , v : & Self) -> Result < () , () > { match self . checked_add (* v) { Some (sum) => { * self = sum ; Ok (()) } None => Err (()) , } } } } ; ($ t : ty , $ ($ tt : ty) ,*) => { impl_weight_int ! ($ t) ; impl_weight_int ! ($ ($ tt) ,*) ; } }
};
}
