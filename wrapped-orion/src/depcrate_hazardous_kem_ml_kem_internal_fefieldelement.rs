// Generated macro for FieldElement (struct)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_feFieldElement {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::fe
// Provides: {"FieldElement"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Debug)] # [doc = " Element in the field Z_q."] # [doc = ""] # [doc = " NOTE(brycx): While for Kyber q = 3329 a field element would fit in u16, but Dilithium q = 8380417 which only fits in u32."] # [doc = " Thus, for possible future re-usability, we use 32-bit integer here."] pub struct FieldElement (pub (crate) u32) ;
};
}
