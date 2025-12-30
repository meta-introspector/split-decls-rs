// Generated macro for impl_1126 (impl)
macro_rules! Depcrate_x509_verifyimpl_1126 {
() => {
// Module: crate::x509::verify
// Provides: {"impl_1126"}
// Dependencies: {}
impl CryptoOps for PyCryptoOps { type Key = pyo3 :: Py < pyo3 :: PyAny > ; type Err = CryptographyError ; type CertificateExtra = pyo3 :: Py < PyCertificate > ; type PolicyExtra = pyo3 :: Py < PyPolicy > ; fn public_key (& self , cert : & Certificate < '_ >) -> Result < Self :: Key , Self :: Err > { pyo3 :: Python :: attach (| py | -> Result < Self :: Key , Self :: Err > { Ok (keys :: load_der_public_key_bytes (py , cert . tbs_cert . spki . tlv () . full_data ()) ? . unbind ()) }) } fn verify_signed_by (& self , cert : & Certificate < '_ > , key : & Self :: Key) -> Result < () , Self :: Err > { pyo3 :: Python :: attach (| py | -> CryptographyResult < () > { sign :: verify_signature_with_signature_algorithm (py , key . bind (py) . clone () , & cert . signature_alg , cert . signature . as_bytes () , & asn1 :: write_single (& cert . tbs_cert) ? ,) }) } fn clone_public_key (key : & Self :: Key) -> Self :: Key { pyo3 :: Python :: attach (| py | key . clone_ref (py)) } fn clone_extra (extra : & Self :: CertificateExtra) -> Self :: CertificateExtra { pyo3 :: Python :: attach (| py | extra . clone_ref (py)) } }
};
}
