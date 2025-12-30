// Generated macro for impl_124 (impl)
macro_rules! Depcrate_backend_dhimpl_124 {
() => {
// Module: crate::backend::dh
// Provides: {"impl_124"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DHPublicNumbers { # [new] fn new (y : pyo3 :: Py < pyo3 :: types :: PyInt > , parameter_numbers : pyo3 :: Py < DHParameterNumbers > ,) -> DHPublicNumbers { DHPublicNumbers { y , parameter_numbers , } } # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] # [pyo3 (signature = (backend = None))] fn public_key (& self , py : pyo3 :: Python < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DHPublicKey > { let _ = backend ; let dh = dh_parameters_from_numbers (py , self . parameter_numbers . get ()) ? ; let pub_key = utils :: py_int_to_bn (py , self . y . bind (py)) ? ; let pkey = openssl :: pkey :: PKey :: from_dh (dh . set_public_key (pub_key) ?) ? ; Ok (DHPublicKey { pkey }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { Ok ((* * self . y . bind (py)) . eq (other . y . bind (py)) ? && self . parameter_numbers . bind (py) . eq (other . parameter_numbers . bind (py)) ?) } }
};
}
