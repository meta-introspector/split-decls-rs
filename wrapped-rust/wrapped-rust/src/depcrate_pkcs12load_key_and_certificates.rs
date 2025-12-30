// Generated macro for load_key_and_certificates (function)
macro_rules! Depcrate_pkcs12load_key_and_certificates {
() => {
// Module: crate::pkcs12
// Provides: {"load_key_and_certificates"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , password , backend = None))] fn load_key_and_certificates < 'p > (py : pyo3 :: Python < 'p > , data : CffiBuf < '_ > , password : Option < CffiBuf < '_ > > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < (pyo3 :: Bound < 'p , pyo3 :: PyAny > , Option < x509 :: certificate :: Certificate > , pyo3 :: Bound < 'p , pyo3 :: types :: PyList > ,) > { let _ = backend ; let p12 = decode_p12 (py , data , password) ? ; let private_key = if let Some (pkey) = p12 . pkey { let pkey_bytes = pkey . private_key_to_pkcs8 () ? ; keys :: load_der_private_key_bytes (py , & pkey_bytes , None , false) ? } else { py . None () . into_bound (py) } ; let cert = if let Some (ossl_cert) = p12 . cert { let cert_der = pyo3 :: types :: PyBytes :: new (py , & ossl_cert . to_der () ?) . unbind () ; Some (x509 :: certificate :: load_der_x509_certificate (py , cert_der , None ,) ?) } else { None } ; let additional_certs = pyo3 :: types :: PyList :: empty (py) ; if let Some (ossl_certs) = p12 . ca { cfg_if :: cfg_if ! { if # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] { let it = ossl_certs . iter () ; } else { let it = ossl_certs . iter () . rev () ; } } ; for ossl_cert in it { let cert_der = pyo3 :: types :: PyBytes :: new (py , & ossl_cert . to_der () ?) . unbind () ; let cert = x509 :: certificate :: load_der_x509_certificate (py , cert_der , None) ? ; additional_certs . append (cert) ? ; } } Ok ((private_key , cert , additional_certs)) }
};
}
