// Generated macro for compute_pss_salt_length (function)
macro_rules! Depcrate_x509_signcompute_pss_salt_length {
() => {
// Module: crate::x509::sign
// Provides: {"compute_pss_salt_length"}
// Dependencies: {}
fn compute_pss_salt_length < 'p > (py : pyo3 :: Python < 'p > , private_key : pyo3 :: Bound < 'p , pyo3 :: PyAny > , hash_algorithm : pyo3 :: Bound < 'p , pyo3 :: PyAny > , rsa_padding : pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < u16 > { let py_saltlen = rsa_padding . getattr (pyo3 :: intern ! (py , "_salt_length")) ? ; if py_saltlen . is_instance (& types :: PADDING_MAX_LENGTH . get (py) ?) ? { types :: CALCULATE_MAX_PSS_SALT_LENGTH . get (py) ? . call1 ((private_key , hash_algorithm)) ? . extract :: < u16 > () } else if py_saltlen . is_instance (& types :: PADDING_DIGEST_LENGTH . get (py) ?) ? { hash_algorithm . getattr (pyo3 :: intern ! (py , "digest_size")) ? . extract :: < u16 > () } else if py_saltlen . is_instance_of :: < pyo3 :: types :: PyInt > () { py_saltlen . extract :: < u16 > () } else { Err (pyo3 :: exceptions :: PyTypeError :: new_err ("salt_length must be an int, MaxLength, or DigestLength." ,)) } }
};
}
