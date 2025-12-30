// Generated macro for impl_100 (impl)
macro_rules! Depcrate_agreementimpl_100 {
() => {
// Module: crate::agreement
// Provides: {"impl_100"}
// Dependencies: {}
impl AsDer < EcPrivateKeyRfc5915Der < 'static > > for PrivateKey { # [doc = " Serializes the key as a DER-encoded `ECPrivateKey` (RFC 5915) structure."] # [doc = ""] # [doc = " X25519 is not supported."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified`  if serialization failed."] fn as_der (& self) -> Result < EcPrivateKeyRfc5915Der < 'static > , Unspecified > { if AlgorithmID :: X25519 == self . inner_key . algorithm () . id { return Err (Unspecified) ; } let mut outp = null_mut :: < u8 > () ; let ec_key = { self . inner_key . get_evp_pkey () . project_const_lifetime (unsafe { | evp_pkey | EVP_PKEY_get0_EC_KEY (* evp_pkey . as_const ()) }) ? } ; let length = usize :: try_from (unsafe { i2d_ECPrivateKey (* ec_key , & mut outp) }) . map_err (| _ | Unspecified) ? ; let mut outp = LcPtr :: new (outp) ? ; Ok (EcPrivateKeyRfc5915Der :: take_from_slice (unsafe { core :: slice :: from_raw_parts_mut (* outp . as_mut () , length) })) } }
};
}
