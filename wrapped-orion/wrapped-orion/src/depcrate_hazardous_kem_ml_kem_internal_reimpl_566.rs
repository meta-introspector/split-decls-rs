// Generated macro for impl_566 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_566 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_566"}
// Dependencies: {}
impl Index < usize > for RingElement { type Output = FieldElement ; fn index (& self , index : usize) -> & Self :: Output { debug_assert ! (index <= 255) ; & self . coefficients [index] } }
};
}
