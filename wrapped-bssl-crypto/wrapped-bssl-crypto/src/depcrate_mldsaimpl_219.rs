// Generated macro for impl_219 (impl)
macro_rules! Depcrate_mldsaimpl_219 {
() => {
// Module: crate::mldsa
// Provides: {"impl_219"}
// Dependencies: {}
impl Prehash87 { # [doc = " Add data to the pre-hashing operation."] pub fn update (& mut self , data : & [u8]) { unsafe { bssl_sys :: MLDSA87_prehash_update (& mut self . 0 , data . as_ffi_ptr () , data . len ()) ; } } # [doc = " Complete the pre-hashing operation."] fn finalize (mut self) -> [u8 ; MU_BYTES] { let mut mu = [0u8 ; MU_BYTES] ; unsafe { bssl_sys :: MLDSA87_prehash_finalize (mu . as_mut_ffi_ptr () , & mut self . 0) ; } mu } }
};
}
