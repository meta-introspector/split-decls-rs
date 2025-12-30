// Generated macro for impl_148 (impl)
macro_rules! Depcrate_backend_dsaimpl_148 {
() => {
// Module: crate::backend::dsa
// Provides: {"impl_148"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DsaPrivateNumbers { # [new] fn new (x : pyo3 :: Py < pyo3 :: types :: PyInt > , public_numbers : pyo3 :: Py < DsaPublicNumbers > ,) -> DsaPrivateNumbers { DsaPrivateNumbers { x , public_numbers } } # [pyo3 (signature = (backend = None))] fn private_key (& self , py : pyo3 :: Python < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DsaPrivateKey > { let _ = backend ; let public_numbers = self . public_numbers . get () ; let parameter_numbers = public_numbers . parameter_numbers . get () ; check_dsa_private_numbers (py , self) ? ; let dsa = openssl :: dsa :: Dsa :: from_private_components (utils :: py_int_to_bn (py , parameter_numbers . p . bind (py)) ? , utils :: py_int_to_bn (py , parameter_numbers . q . bind (py)) ? , utils :: py_int_to_bn (py , parameter_numbers . g . bind (py)) ? , utils :: py_int_to_bn (py , self . x . bind (py)) ? , utils :: py_int_to_bn (py , public_numbers . y . bind (py)) ? ,) . unwrap () ; let pkey = openssl :: pkey :: PKey :: from_dsa (dsa) ? ; Ok (DsaPrivateKey { pkey }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { Ok ((* * self . x . bind (py)) . eq (other . x . bind (py)) ? && self . public_numbers . bind (py) . eq (other . public_numbers . bind (py)) ?) } }
};
}
