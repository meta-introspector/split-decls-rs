// Generated macro for impl_442 (impl)
macro_rules! Depcrate_declarative_asn1_typesimpl_442 {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"impl_442"}
// Dependencies: {}
# [pyo3 :: pymethods] impl Annotation { # [new] # [pyo3 (signature = (default = None , encoding = None , size = None))] fn new (default : Option < pyo3 :: Py < pyo3 :: types :: PyAny > > , encoding : Option < pyo3 :: Py < Encoding > > , size : Option < pyo3 :: Py < Size > > ,) -> Self { Self { default , encoding , size , } } # [pyo3 (signature = ())] fn is_empty (& self) -> bool { self . default . is_none () && self . encoding . is_none () && self . size . is_none () } }
};
}
