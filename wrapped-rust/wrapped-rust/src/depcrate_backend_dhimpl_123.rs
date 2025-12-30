// Generated macro for impl_123 (impl)
macro_rules! Depcrate_backend_dhimpl_123 {
() => {
// Module: crate::backend::dh
// Provides: {"impl_123"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DHPrivateNumbers { # [new] fn new (x : pyo3 :: Py < pyo3 :: types :: PyInt > , public_numbers : pyo3 :: Py < DHPublicNumbers > ,) -> DHPrivateNumbers { DHPrivateNumbers { x , public_numbers } } # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] # [pyo3 (signature = (backend = None))] fn private_key (& self , py : pyo3 :: Python < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DHPrivateKey > { let _ = backend ; let dh = dh_parameters_from_numbers (py , self . public_numbers . get () . parameter_numbers . get ()) ? ; let pub_key = utils :: py_int_to_bn (py , self . public_numbers . get () . y . bind (py)) ? ; let priv_key = utils :: py_int_to_bn (py , self . x . bind (py)) ? ; let dh = dh . set_key (pub_key , priv_key) ? ; let pkey = openssl :: pkey :: PKey :: from_dh (dh) ? ; Ok (DHPrivateKey { pkey }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { Ok ((* * self . x . bind (py)) . eq (other . x . bind (py)) ? && self . public_numbers . bind (py) . eq (other . public_numbers . bind (py)) ?) } }
};
}
