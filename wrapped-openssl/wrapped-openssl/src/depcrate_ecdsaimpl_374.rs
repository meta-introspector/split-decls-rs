// Generated macro for impl_374 (impl)
macro_rules! Depcrate_ecdsaimpl_374 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_374"}
// Dependencies: {}
impl EcdsaSig { # [doc = " Computes a digital signature of the hash value `data` using the private EC key eckey."] # [corresponds (ECDSA_do_sign)] pub fn sign < T > (data : & [u8] , eckey : & EcKeyRef < T >) -> Result < EcdsaSig , ErrorStack > where T : HasPrivate , { unsafe { assert ! (data . len () <= c_int :: MAX as usize) ; let sig = cvt_p (ffi :: ECDSA_do_sign (data . as_ptr () , data . len () as LenType , eckey . as_ptr () ,)) ? ; Ok (EcdsaSig :: from_ptr (sig)) } } # [doc = " Returns a new `EcdsaSig` by setting the `r` and `s` values associated with an ECDSA signature."] # [corresponds (ECDSA_SIG_set0)] pub fn from_private_components (r : BigNum , s : BigNum) -> Result < EcdsaSig , ErrorStack > { unsafe { let sig = cvt_p (ffi :: ECDSA_SIG_new ()) ? ; ECDSA_SIG_set0 (sig , r . as_ptr () , s . as_ptr ()) ; mem :: forget ((r , s)) ; Ok (EcdsaSig :: from_ptr (sig)) } } from_der ! { # [doc = " Decodes a DER-encoded ECDSA signature."] # [corresponds (d2i_ECDSA_SIG)] from_der , EcdsaSig , ffi :: d2i_ECDSA_SIG } }
};
}
