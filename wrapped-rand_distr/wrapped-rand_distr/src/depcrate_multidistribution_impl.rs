// Generated macro for distribution_impl (macro)
macro_rules! Depcrate_multidistribution_impl {
() => {
// Module: crate::multi
// Provides: {"distribution_impl"}
// Dependencies: {}
macro_rules ! distribution_impl { ($ scalar : ident) => { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Vec <$ scalar > { use crate :: multi :: MultiDistribution ; let mut buf = vec ! [Default :: default () ; self . sample_len ()] ; self . sample_to_slice (rng , & mut buf) ; buf } } ; }
};
}
