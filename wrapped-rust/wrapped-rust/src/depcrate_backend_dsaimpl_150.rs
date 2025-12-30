// Generated macro for impl_150 (impl)
macro_rules! Depcrate_backend_dsaimpl_150 {
() => {
// Module: crate::backend::dsa
// Provides: {"impl_150"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DsaParameterNumbers { # [new] fn new (p : pyo3 :: Py < pyo3 :: types :: PyInt > , q : pyo3 :: Py < pyo3 :: types :: PyInt > , g : pyo3 :: Py < pyo3 :: types :: PyInt > ,) -> DsaParameterNumbers { DsaParameterNumbers { p , q , g } } # [pyo3 (signature = (backend = None))] fn parameters (& self , py : pyo3 :: Python < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DsaParameters > { let _ = backend ; check_dsa_parameters (py , self) ? ; let dsa = openssl :: dsa :: Dsa :: from_pqg (utils :: py_int_to_bn (py , self . p . bind (py)) ? , utils :: py_int_to_bn (py , self . q . bind (py)) ? , utils :: py_int_to_bn (py , self . g . bind (py)) ? ,) . unwrap () ; Ok (DsaParameters { dsa }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { Ok ((* * self . p . bind (py)) . eq (other . p . bind (py)) ? && (* * self . q . bind (py)) . eq (other . q . bind (py)) ? && (* * self . g . bind (py)) . eq (other . g . bind (py)) ?) } fn __repr__ (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { let p = self . p . bind (py) ; let q = self . q . bind (py) ; let g = self . g . bind (py) ; Ok (format ! ("<DSAParameterNumbers(p={p}, q={q}, g={g})>")) } }
};
}
