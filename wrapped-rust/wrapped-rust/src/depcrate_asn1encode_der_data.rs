// Generated macro for encode_der_data (function)
macro_rules! Depcrate_asn1encode_der_data {
() => {
// Module: crate::asn1
// Provides: {"encode_der_data"}
// Dependencies: {}
pub (crate) fn encode_der_data < 'p > (py : pyo3 :: Python < 'p > , pem_tag : String , data : Vec < u8 > , encoding : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { if encoding . is (& types :: ENCODING_DER . get (py) ?) { Ok (pyo3 :: types :: PyBytes :: new (py , & data)) } else if encoding . is (& types :: ENCODING_PEM . get (py) ?) { Ok (pyo3 :: types :: PyBytes :: new (py , & pem :: encode_config (& pem :: Pem :: new (pem_tag , data) , cryptography_key_parsing :: pem :: ENCODE_CONFIG ,) . into_bytes () ,)) } else { Err (pyo3 :: exceptions :: PyTypeError :: new_err ("encoding must be Encoding.DER or Encoding.PEM") . into () ,) } }
};
}
