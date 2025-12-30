// Generated macro for impl_343 (impl)
macro_rules! Depcrate_backend_rsaimpl_343 {
() => {
// Module: crate::backend::rsa
// Provides: {"impl_343"}
// Dependencies: {}
# [pyo3 :: pymethods] impl RsaPublicNumbers { # [new] fn new (e : pyo3 :: Py < pyo3 :: types :: PyInt > , n : pyo3 :: Py < pyo3 :: types :: PyInt >) -> RsaPublicNumbers { RsaPublicNumbers { e , n } } # [pyo3 (signature = (backend = None))] fn public_key (& self , py : pyo3 :: Python < '_ > , backend : Option < & pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < RsaPublicKey > { let _ = backend ; check_public_key_components (self . e . bind (py) , self . n . bind (py)) ? ; let rsa = openssl :: rsa :: Rsa :: from_public_components (utils :: py_int_to_bn (py , self . n . bind (py)) ? , utils :: py_int_to_bn (py , self . e . bind (py)) ? ,) . unwrap () ; let pkey = openssl :: pkey :: PKey :: from_rsa (rsa) ? ; Ok (RsaPublicKey { pkey }) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { Ok ((* * self . e . bind (py)) . eq (other . e . bind (py)) ? && (* * self . n . bind (py)) . eq (other . n . bind (py)) ? ,) } fn __hash__ (& self , py : pyo3 :: Python < '_ >) -> CryptographyResult < u64 > { let mut hasher = DefaultHasher :: new () ; self . e . bind (py) . hash () ? . hash (& mut hasher) ; self . n . bind (py) . hash () ? . hash (& mut hasher) ; Ok (hasher . finish ()) } fn __repr__ (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { let e = self . e . bind (py) ; let n = self . n . bind (py) ; Ok (format ! ("<RSAPublicNumbers(e={e}, n={n})>")) } }
};
}
