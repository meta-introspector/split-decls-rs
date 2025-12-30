// Generated macro for identify_key_type (function)
macro_rules! Depcrate_x509_signidentify_key_type {
() => {
// Module: crate::x509::sign
// Provides: {"identify_key_type"}
// Dependencies: {}
pub (crate) fn identify_key_type (py : pyo3 :: Python < '_ > , private_key : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < KeyType > { if private_key . is_instance (& types :: RSA_PRIVATE_KEY . get (py) ?) ? { Ok (KeyType :: Rsa) } else if private_key . is_instance (& types :: DSA_PRIVATE_KEY . get (py) ?) ? { Ok (KeyType :: Dsa) } else if private_key . is_instance (& types :: ELLIPTIC_CURVE_PRIVATE_KEY . get (py) ?) ? { Ok (KeyType :: Ec) } else if private_key . is_instance (& types :: ED25519_PRIVATE_KEY . get (py) ?) ? { Ok (KeyType :: Ed25519) } else if private_key . is_instance (& types :: ED448_PRIVATE_KEY . get (py) ?) ? { Ok (KeyType :: Ed448) } else { Err (pyo3 :: exceptions :: PyTypeError :: new_err ("Key must be an rsa, dsa, ec, ed25519, or ed448 private key." ,)) } }
};
}
