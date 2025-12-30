// Generated macro for symmetric_decrypt (function)
macro_rules! Depcrate_pkcs7symmetric_decrypt {
() => {
// Module: crate::pkcs7
// Provides: {"symmetric_decrypt"}
// Dependencies: {}
pub (crate) fn symmetric_decrypt (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode : pyo3 :: Bound < '_ , pyo3 :: PyAny > , data : & [u8] ,) -> CryptographyResult < Vec < u8 > > { let block_size = algorithm . getattr (pyo3 :: intern ! (py , "block_size")) ? . extract () ? ; let mut cipher = ciphers :: CipherContext :: new (py , algorithm , mode , openssl :: symm :: Mode :: Decrypt) ? ; let mut decrypted_data = vec ! [0 ; data . len () + (block_size / 8)] ; let count = cipher . update_into (py , data , & mut decrypted_data) ? ; let final_block = cipher . finalize (py) ? ; assert ! (final_block . as_bytes () . is_empty ()) ; decrypted_data . truncate (count) ; let mut unpadder = PKCS7UnpaddingContext :: new (block_size) ; let unpadded_first_blocks = unpadder . update (py , CffiBuf :: from_bytes (py , & decrypted_data)) ? ; let unpadded_last_block = unpadder . finalize (py) ? ; let unpadded_data = [unpadded_first_blocks . as_bytes () , unpadded_last_block . as_bytes () ,] . concat () ; Ok (unpadded_data) }
};
}
