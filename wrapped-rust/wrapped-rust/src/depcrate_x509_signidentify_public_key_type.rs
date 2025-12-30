// Generated macro for identify_public_key_type (function)
macro_rules! Depcrate_x509_signidentify_public_key_type {
() => {
// Module: crate::x509::sign
// Provides: {"identify_public_key_type"}
// Dependencies: {}
pub (crate) fn identify_public_key_type (py : pyo3 :: Python < '_ > , public_key : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < KeyType > { if public_key . is_instance (& types :: RSA_PUBLIC_KEY . get (py) ?) ? { Ok (KeyType :: Rsa) } else if public_key . is_instance (& types :: DSA_PUBLIC_KEY . get (py) ?) ? { Ok (KeyType :: Dsa) } else if public_key . is_instance (& types :: ELLIPTIC_CURVE_PUBLIC_KEY . get (py) ?) ? { Ok (KeyType :: Ec) } else if public_key . is_instance (& types :: ED25519_PUBLIC_KEY . get (py) ?) ? { Ok (KeyType :: Ed25519) } else if public_key . is_instance (& types :: ED448_PUBLIC_KEY . get (py) ?) ? { Ok (KeyType :: Ed448) } else { Err (pyo3 :: exceptions :: PyTypeError :: new_err ("Key must be an rsa, dsa, ec, ed25519, or ed448 public key." ,)) } }
};
}
