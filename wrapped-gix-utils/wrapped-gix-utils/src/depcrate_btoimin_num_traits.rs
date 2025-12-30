// Generated macro for min_num_traits (macro)
macro_rules! Depcrate_btoimin_num_traits {
() => {
// Module: crate::btoi
// Provides: {"min_num_traits"}
// Dependencies: {}
macro_rules ! min_num_traits { ($ t : ty) => { impl MinNumTraits for $ t { const ZERO : Self = 0 ; impl_checked ! (checked_add) ; impl_checked ! (checked_mul) ; impl_checked ! (checked_sub) ; } } ; }
};
}
