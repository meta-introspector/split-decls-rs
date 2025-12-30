// Generated macro for pkcs7_verify (function)
macro_rules! Depcrate_test_supportpkcs7_verify {
() => {
// Module: crate::test_support
// Provides: {"pkcs7_verify"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] # [pyo3 :: pyfunction] # [pyo3 (signature = (encoding , sig , msg , certs , options))] fn pkcs7_verify (py : pyo3 :: Python < '_ > , encoding : pyo3 :: Bound < '_ , pyo3 :: PyAny > , sig : & [u8] , msg : Option < CffiBuf < '_ > > , certs : Vec < pyo3 :: Py < PyCertificate > > , options : pyo3 :: Bound < '_ , pyo3 :: types :: PyList > ,) -> CryptographyResult < () > { let p7 = if encoding . is (& types :: ENCODING_DER . get (py) ?) { openssl :: pkcs7 :: Pkcs7 :: from_der (sig) ? } else if encoding . is (& types :: ENCODING_PEM . get (py) ?) { openssl :: pkcs7 :: Pkcs7 :: from_pem (sig) ? } else { openssl :: pkcs7 :: Pkcs7 :: from_smime (sig) ? . 0 } ; let mut flags = openssl :: pkcs7 :: Pkcs7Flags :: empty () ; if options . contains (types :: PKCS7_TEXT . get (py) ?) ? { flags |= openssl :: pkcs7 :: Pkcs7Flags :: TEXT ; } let store = { let mut b = openssl :: x509 :: store :: X509StoreBuilder :: new () ? ; for cert in & certs { let der = asn1 :: write_single (cert . get () . raw . borrow_dependent ()) ? ; b . add_cert (openssl :: x509 :: X509 :: from_der (& der) ?) ? ; } b . build () } ; let certs = openssl :: stack :: Stack :: new () ? ; p7 . verify (& certs , & store , msg . as_ref () . map (| m | m . as_bytes ()) , None , flags ,) ? ; Ok (()) }
};
}
