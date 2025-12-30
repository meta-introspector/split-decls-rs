// Generated macro for const_distribution_impl (macro)
macro_rules! Depcrate_multiconst_distribution_impl {
() => {
// Module: crate::multi
// Provides: {"const_distribution_impl"}
// Dependencies: {}
# [allow (unused)] macro_rules ! const_distribution_impl { ($ scalar : ident) => { fn sample < R : Rng + ? Sized > (& self , rng : & mut R ,) -> [$ scalar ; < Self as crate :: multi :: MultiDistribution >:: SAMPLE_LEN] { use crate :: multi :: MultiDistribution ; let mut buf = [Default :: default () ; < Self as crate :: multi :: MultiDistribution >:: SAMPLE_LEN] ; self . sample_to_slice (rng , & mut buf) ; buf } } ; }
};
}
