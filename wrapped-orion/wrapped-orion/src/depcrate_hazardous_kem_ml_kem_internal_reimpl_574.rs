// Generated macro for impl_574 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_574 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_574"}
// Dependencies: {}
impl IndexMut < usize > for RingElementNTT { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { debug_assert ! (index <= 255) ; & mut self . coefficients [index] } }
};
}
