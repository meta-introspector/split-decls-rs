// Generated macro for message_digest_from_algorithm (function)
macro_rules! Depcrate_backend_hashesmessage_digest_from_algorithm {
() => {
// Module: crate::backend::hashes
// Provides: {"message_digest_from_algorithm"}
// Dependencies: {}
pub (crate) fn message_digest_from_algorithm (py : pyo3 :: Python < '_ > , algorithm : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < openssl :: hash :: MessageDigest > { if ! algorithm . is_instance (& types :: HASH_ALGORITHM . get (py) ?) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyTypeError :: new_err ("Expected instance of hashes.HashAlgorithm.") ,)) ; } let name = algorithm . getattr (pyo3 :: intern ! (py , "name")) ? . extract :: < pyo3 :: pybacked :: PyBackedStr > () ? ; let openssl_name = if name == "blake2b" || name == "blake2s" { let digest_size = algorithm . getattr (pyo3 :: intern ! (py , "digest_size")) ? . extract :: < usize > () ? ; Cow :: Owned (format ! ("{}{}" , name , digest_size * 8)) } else { Cow :: Borrowed (name . as_ref ()) } ; match openssl :: hash :: MessageDigest :: from_name (& openssl_name) { Some (md) => Ok (md) , None => Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err ((format ! ("{name} is not a supported hash on this backend") , exceptions :: Reasons :: UNSUPPORTED_HASH ,)) ,)) , } }
};
}
