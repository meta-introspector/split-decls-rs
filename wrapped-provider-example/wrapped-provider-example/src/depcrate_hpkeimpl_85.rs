// Generated macro for impl_85 (impl)
macro_rules! Depcrate_hpkeimpl_85 {
() => {
// Module: crate::hpke
// Provides: {"impl_85"}
// Dependencies: {}
impl HpkeOpener for HpkeRsReceiver { fn open (& mut self , aad : & [u8] , ciphertext : & [u8]) -> Result < Vec < u8 > , Error > { self . context . open (aad , ciphertext) . map_err (other_err) } }
};
}
