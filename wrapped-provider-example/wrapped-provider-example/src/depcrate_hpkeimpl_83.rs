// Generated macro for impl_83 (impl)
macro_rules! Depcrate_hpkeimpl_83 {
() => {
// Module: crate::hpke
// Provides: {"impl_83"}
// Dependencies: {}
impl HpkeSealer for HpkeRsSender { fn seal (& mut self , aad : & [u8] , plaintext : & [u8]) -> Result < Vec < u8 > , Error > { self . context . seal (aad , plaintext) . map_err (other_err) } }
};
}
