// Generated macro for serialize_certificates (function)
macro_rules! Depcrate_pkcs7serialize_certificates {
() => {
// Module: crate::pkcs7
// Provides: {"serialize_certificates"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn serialize_certificates < 'p > (py : pyo3 :: Python < 'p > , py_certs : Vec < pyo3 :: PyRef < 'p , x509 :: certificate :: Certificate > > , encoding : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { if py_certs . is_empty () { return Err (pyo3 :: exceptions :: PyTypeError :: new_err ("certs must be a list of certs with length >= 1" ,) . into ()) ; } let raw_certs = py_certs . iter () . map (| c | c . raw . borrow_dependent () . clone ()) . collect :: < Vec < _ > > () ; let signed_data = pkcs7 :: SignedData { version : 1 , digest_algorithms : common :: Asn1ReadableOrWritable :: new_write (asn1 :: SetOfWriter :: new (& [])) , content_info : pkcs7 :: ContentInfo { _content_type : asn1 :: DefinedByMarker :: marker () , content : pkcs7 :: Content :: Data (None) , } , certificates : Some (common :: Asn1ReadableOrWritable :: new_write (asn1 :: SetOfWriter :: new (& raw_certs) ,)) , crls : None , signer_infos : common :: Asn1ReadableOrWritable :: new_write (asn1 :: SetOfWriter :: new (& [])) , } ; let content_info = pkcs7 :: ContentInfo { _content_type : asn1 :: DefinedByMarker :: marker () , content : pkcs7 :: Content :: SignedData (asn1 :: Explicit :: new (Box :: new (signed_data))) , } ; let content_info_bytes = asn1 :: write_single (& content_info) ? ; encode_der_data (py , "PKCS7" . to_string () , content_info_bytes , encoding) }
};
}
