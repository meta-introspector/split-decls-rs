// Generated macro for impl_375 (impl)
macro_rules! Depcrate_ecdsaimpl_375 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_375"}
// Dependencies: {}
impl EcdsaSigRef { to_der ! { # [doc = " Serializes the ECDSA signature into a DER-encoded ECDSASignature structure."] # [corresponds (i2d_ECDSA_SIG)] to_der , ffi :: i2d_ECDSA_SIG } # [doc = " Verifies if the signature is a valid ECDSA signature using the given public key."] # [corresponds (ECDSA_do_verify)] pub fn verify < T > (& self , data : & [u8] , eckey : & EcKeyRef < T >) -> Result < bool , ErrorStack > where T : HasPublic , { unsafe { assert ! (data . len () <= c_int :: MAX as usize) ; cvt_n (ffi :: ECDSA_do_verify (data . as_ptr () , data . len () as LenType , self . as_ptr () , eckey . as_ptr () ,)) . map (| x | x == 1) } } # [doc = " Returns internal component: `r` of an `EcdsaSig`. (See X9.62 or FIPS 186-2)"] # [corresponds (ECDSA_SIG_get0)] pub fn r (& self) -> & BigNumRef { unsafe { let mut r = ptr :: null () ; ECDSA_SIG_get0 (self . as_ptr () , & mut r , ptr :: null_mut ()) ; BigNumRef :: from_const_ptr (r) } } # [doc = " Returns internal components: `s` of an `EcdsaSig`. (See X9.62 or FIPS 186-2)"] # [corresponds (ECDSA_SIG_get0)] pub fn s (& self) -> & BigNumRef { unsafe { let mut s = ptr :: null () ; ECDSA_SIG_get0 (self . as_ptr () , ptr :: null_mut () , & mut s) ; BigNumRef :: from_const_ptr (s) } } }
};
}
