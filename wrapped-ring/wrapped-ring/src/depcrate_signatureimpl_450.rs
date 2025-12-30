// Generated macro for impl_450 (impl)
macro_rules! Depcrate_signatureimpl_450 {
() => {
// Module: crate::signature
// Provides: {"impl_450"}
// Dependencies: {}
impl Signature { pub (crate) fn new < F > (fill : F) -> Self where F : FnOnce (& mut [u8 ; MAX_LEN]) -> usize , { let mut r = Self { value : [0 ; MAX_LEN] , len : 0 , } ; r . len = fill (& mut r . value) ; r } }
};
}
