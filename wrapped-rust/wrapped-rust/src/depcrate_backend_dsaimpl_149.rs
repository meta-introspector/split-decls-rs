// Generated macro for impl_149 (impl)
macro_rules! Depcrate_backend_dsaimpl_149 {
() => {
// Module: crate::backend::dsa
// Provides: {"impl_149"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DsaPublicNumbers { # [new] fn new (y : pyo3 :: Py < pyo3 :: types :: PyInt > , parameter_numbers : pyo3 :: Py < DsaParameterNumbers > ,) -> DsaPublicNumbers { DsaPublicNumbers { y , parameter_numbers , } } # [pyo3 (signature = (backend = None))] fn public_key (& self , py : pyo3 :: Python < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DsaPublicKey > { let _ = backend ; let parameter_numbers = self . parameter_numbers . get () ; check_dsa_parameters (py , parameter_numbers) ? ; let dsa = openssl :: dsa :: Dsa :: from_public_components (utils :: py_int_to_bn (py , parameter_numbers . p . bind (py)) ? , utils :: py_int_to_bn (py , parameter_numbers . q . bind (py)) ? , utils :: py_int_to_bn (py , parameter_numbers . g . bind (py)) ? , utils :: py_int_to_bn (py , self . y . bind (py)) ? ,) . unwrap () ; let pkey = openssl :: pkey :: PKey :: from_dsa (dsa) ? ; Ok (DsaPublicKey { pkey }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { Ok ((* * self . y . bind (py)) . eq (other . y . bind (py)) ? && self . parameter_numbers . bind (py) . eq (other . parameter_numbers . bind (py)) ?) } fn __repr__ (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { let y = self . y . bind (py) ; let parameter_numbers = self . parameter_numbers . bind (py) . repr () ? ; Ok (format ! ("<DSAPublicNumbers(y={y}, parameter_numbers={parameter_numbers})>")) } }
};
}
