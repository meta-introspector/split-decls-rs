// Generated macro for symmetric_encrypt (function)
macro_rules! Depcrate_pkcs12symmetric_encrypt {
() => {
// Module: crate::pkcs12
// Provides: {"symmetric_encrypt"}
// Dependencies: {}
pub (crate) fn symmetric_encrypt (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode : pyo3 :: Bound < '_ , pyo3 :: PyAny > , data : & [u8] ,) -> CryptographyResult < Vec < u8 > > { let block_size = algorithm . getattr (pyo3 :: intern ! (py , "block_size")) ? . extract () ? ; let mut cipher = ciphers :: CipherContext :: new (py , algorithm , mode , openssl :: symm :: Mode :: Encrypt) ? ; let mut ciphertext = vec ! [0 ; data . len () + (block_size / 8 * 2)] ; let n = cipher . update_into (py , data , & mut ciphertext) ? ; let mut padder = PKCS7PaddingContext :: new (block_size) ; assert ! (padder . update (CffiBuf :: from_bytes (py , data)) ?. is_none ()) ; let padding = padder . finalize (py) ? ; let pad_n = cipher . update_into (py , padding . as_bytes () , & mut ciphertext [n ..]) ? ; let final_block = cipher . finalize (py) ? ; assert ! (final_block . as_bytes () . is_empty ()) ; ciphertext . truncate (n + pad_n) ; Ok (ciphertext) }
};
}
