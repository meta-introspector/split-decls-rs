// Generated macro for create_revoked_certificate (function)
macro_rules! Depcrate_x509_crlcreate_revoked_certificate {
() => {
// Module: crate::x509::crl
// Provides: {"create_revoked_certificate"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn create_revoked_certificate (py : pyo3 :: Python < '_ > , builder : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < RevokedCertificate > { let serial_number = builder . getattr (pyo3 :: intern ! (py , "_serial_number")) ? . extract () ? ; let py_revocation_date = builder . getattr (pyo3 :: intern ! (py , "_revocation_date")) ? . extract () ? ; let ka_vec = cryptography_keepalive :: KeepAlive :: new () ; let ka_bytes = cryptography_keepalive :: KeepAlive :: new () ; let serial_bytes = py_uint_to_big_endian_bytes (py , serial_number) ? ; let revoked_cert = crl :: RevokedCertificate { user_certificate : SerialNumber :: new (& serial_bytes) . unwrap () , revocation_date : x509 :: certificate :: time_from_py (py , & py_revocation_date) ? , raw_crl_entry_extensions : x509 :: common :: encode_extensions (py , & ka_vec , & ka_bytes , & builder . getattr (pyo3 :: intern ! (py , "_extensions")) ? , extensions :: encode_extension ,) ? , } ; let data = asn1 :: write_single (& revoked_cert) ? ; let owned = OwnedRevokedCertificate :: try_new (pyo3 :: types :: PyBytes :: new (py , & data) . unbind () , | data | { asn1 :: parse_single (data . as_bytes (py)) }) ? ; Ok (RevokedCertificate { owned , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) }
};
}
