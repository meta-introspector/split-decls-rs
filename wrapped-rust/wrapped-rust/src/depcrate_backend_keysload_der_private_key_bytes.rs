// Generated macro for load_der_private_key_bytes (function)
macro_rules! Depcrate_backend_keysload_der_private_key_bytes {
() => {
// Module: crate::backend::keys
// Provides: {"load_der_private_key_bytes"}
// Dependencies: {}
pub (crate) fn load_der_private_key_bytes < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] , password : Option < & [u8] > , unsafe_skip_rsa_key_validation : bool ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let parsers : [fn (& [u8]) -> cryptography_key_parsing :: KeyParsingResult < _ > ; 4] = [cryptography_key_parsing :: pkcs8 :: parse_private_key , | d | cryptography_key_parsing :: ec :: parse_pkcs1_private_key (d , None) , cryptography_key_parsing :: rsa :: parse_pkcs1_private_key , cryptography_key_parsing :: dsa :: parse_pkcs1_private_key ,] ; let pkey = parsers . iter () . find_map (| parser | match parser (data) { Ok (key) => Some (Ok (key)) , Err (cryptography_key_parsing :: KeyParsingError :: Parse (_)) => None , Err (e) => Some (Err (e)) , }) ; if let Some (Ok (pkey)) = pkey { if password . is_some () { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyTypeError :: new_err ("Password was given but private key is not encrypted." ,) ,)) ; } return private_key_from_pkey (py , & pkey , unsafe_skip_rsa_key_validation) ; } else if let Some (Err (e)) = pkey { return Err (e . into ()) ; } let pkey = cryptography_key_parsing :: pkcs8 :: parse_encrypted_private_key (data , password) ? ; private_key_from_pkey (py , & pkey , unsafe_skip_rsa_key_validation) }
};
}
