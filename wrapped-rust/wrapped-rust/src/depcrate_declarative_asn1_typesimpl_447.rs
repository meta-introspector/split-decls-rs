// Generated macro for impl_447 (impl)
macro_rules! Depcrate_declarative_asn1_typesimpl_447 {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"impl_447"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PrintableString { # [new] # [pyo3 (signature = (inner ,))] fn new (py : pyo3 :: Python < '_ > , inner : pyo3 :: Py < pyo3 :: types :: PyString >) -> pyo3 :: PyResult < Self > { if Asn1PrintableString :: new (& inner . to_cow (py) ?) . is_none () { return Err (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("invalid PrintableString: {inner}"))) ; } Ok (PrintableString { inner }) } # [pyo3 (signature = ())] pub fn as_str (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < pyo3 :: Py < pyo3 :: types :: PyString > > { Ok (self . inner . clone_ref (py)) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self >) -> pyo3 :: PyResult < bool > { (* * self . inner . bind (py)) . eq (other . inner . bind (py)) } pub fn __repr__ (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { Ok (format ! ("PrintableString({})" , self . inner . bind (py) . repr () ?)) } }
};
}
