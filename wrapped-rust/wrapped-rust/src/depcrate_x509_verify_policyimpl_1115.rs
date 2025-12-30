// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_x509_verify_policyimpl_1115 {
() => {
// Module: crate::x509::verify::policy
// Provides: {"impl_1115"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PyPolicy { # [getter] pub (super) fn max_chain_depth (& self) -> u8 { self . policy_definition . borrow_dependent () . max_chain_depth } # [getter] pub (super) fn subject (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: Py < pyo3 :: PyAny > { self . subject . clone_ref (py) } # [getter] pub (super) fn validation_time (& self , py : pyo3 :: Python < '_ > ,) -> pyo3 :: PyResult < pyo3 :: Py < pyo3 :: PyAny > > { let time = & self . policy_definition . borrow_dependent () . validation_time ; Ok (datetime_to_py (py , time) ? . as_unbound () . clone_ref (py)) } # [getter] fn extended_key_usage (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < pyo3 :: Py < pyo3 :: PyAny > > { let eku_oid = & self . policy_definition . borrow_dependent () . extended_key_usage ; Ok (oid_to_py_oid (py , eku_oid) ? . as_unbound () . clone_ref (py)) } # [getter] fn minimum_rsa_modulus (& self) -> usize { self . policy_definition . borrow_dependent () . minimum_rsa_modulus } }
};
}
