// Generated macro for impl_567 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_567 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_567"}
// Dependencies: {}
impl IndexMut < usize > for RingElement { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { debug_assert ! (index <= 255) ; & mut self . coefficients [index] } }
};
}
