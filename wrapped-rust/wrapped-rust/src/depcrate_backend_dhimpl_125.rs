// Generated macro for impl_125 (impl)
macro_rules! Depcrate_backend_dhimpl_125 {
() => {
// Module: crate::backend::dh
// Provides: {"impl_125"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DHParameterNumbers { # [new] # [pyo3 (signature = (p , g , q = None))] fn new (py : pyo3 :: Python < '_ > , p : pyo3 :: Py < pyo3 :: types :: PyInt > , g : pyo3 :: Py < pyo3 :: types :: PyInt > , q : Option < pyo3 :: Py < pyo3 :: types :: PyInt > > ,) -> CryptographyResult < DHParameterNumbers > { if g . bind (py) . lt (2) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("DH generator must be 2 or greater") ,)) ; } if p . bind (py) . call_method0 ("bit_length") ? . lt (cryptography_key_parsing :: MIN_DH_MODULUS_SIZE) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("p (modulus) must be at least {}-bit" , cryptography_key_parsing :: MIN_DH_MODULUS_SIZE)) ,)) ; } Ok (DHParameterNumbers { p , g , q }) } # [pyo3 (signature = (backend = None))] fn parameters (& self , py : pyo3 :: Python < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DHParameters > { let _ = backend ; let dh = dh_parameters_from_numbers (py , self) ? ; Ok (DHParameters { dh }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { let q_equal = match (self . q . as_ref () , other . q . as_ref ()) { (Some (self_q) , Some (other_q)) => (* * self_q . bind (py)) . eq (other_q . bind (py)) ? , (None , None) => true , _ => false , } ; Ok ((* * self . p . bind (py)) . eq (other . p . bind (py)) ? && (* * self . g . bind (py)) . eq (other . g . bind (py)) ? && q_equal) } }
};
}
