// Generated macro for sign_data (function)
macro_rules! Depcrate_x509_signsign_data {
() => {
// Module: crate::x509::sign
// Provides: {"sign_data"}
// Dependencies: {}
pub (crate) fn sign_data < 'p > (py : pyo3 :: Python < 'p > , private_key : pyo3 :: Bound < 'p , pyo3 :: PyAny > , hash_algorithm : pyo3 :: Bound < 'p , pyo3 :: PyAny > , rsa_padding : pyo3 :: Bound < 'p , pyo3 :: PyAny > , ecdsa_deterministic : Option < bool > , data : & [u8] ,) -> pyo3 :: PyResult < PyBackedBytes > { let key_type = identify_key_type (py , private_key . clone ()) ? ; let signature = match key_type { KeyType :: Ed25519 | KeyType :: Ed448 => { private_key . call_method1 (pyo3 :: intern ! (py , "sign") , (data ,)) ? } KeyType :: Ec => { let ecdsa = types :: ECDSA . get (py) ? . call1 ((hash_algorithm , ecdsa_deterministic . unwrap_or (false))) ? ; private_key . call_method1 (pyo3 :: intern ! (py , "sign") , (data , ecdsa)) ? } KeyType :: Rsa => { let mut padding = rsa_padding ; if padding . is_none () { padding = types :: PKCS1V15 . get (py) ? . call0 () ? ; } private_key . call_method1 (pyo3 :: intern ! (py , "sign") , (data , padding , hash_algorithm)) ? } KeyType :: Dsa => { private_key . call_method1 (pyo3 :: intern ! (py , "sign") , (data , hash_algorithm)) ? } } ; Ok (signature . extract () ?) }
};
}
