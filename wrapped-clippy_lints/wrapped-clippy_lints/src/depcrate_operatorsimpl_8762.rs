// Generated macro for impl_8762 (impl)
macro_rules! Depcrate_operatorsimpl_8762 {
() => {
// Module: crate::operators
// Provides: {"impl_8762"}
// Dependencies: {}
impl Operators { pub fn new (conf : & 'static Conf) -> Self { Self { arithmetic_context : numeric_arithmetic :: Context :: default () , verbose_bit_mask_threshold : conf . verbose_bit_mask_threshold , modulo_arithmetic_allow_comparison_to_zero : conf . allow_comparison_to_zero , msrv : conf . msrv , } } }
};
}
