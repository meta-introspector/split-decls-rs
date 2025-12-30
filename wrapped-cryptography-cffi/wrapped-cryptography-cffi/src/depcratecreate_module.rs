// Generated macro for create_module (function)
macro_rules! Depcratecreate_module {
() => {
// Module: crate
// Provides: {"create_module"}
// Dependencies: {}
pub fn create_module (py : pyo3 :: Python < '_ > ,) -> pyo3 :: PyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyModule > > { # [cfg (python_implementation = "PyPy")] let openssl_mod = unsafe { let res = Cryptography_make_openssl_module () ; assert_eq ! (res , 0) ; pyo3 :: types :: PyModule :: import (py , "_openssl") ? . clone () } ; # [cfg (not (python_implementation = "PyPy"))] let openssl_mod = unsafe { let ptr = PyInit__openssl () ; pyo3 :: Py :: from_owned_ptr_or_err (py , ptr) ? . bind (py) . clone () } ; Ok (openssl_mod) }
};
}
