// Generated macro for impl_909 (impl)
macro_rules! Depcrate_x509_crlimpl_909 {
() => {
// Module: crate::x509::crl
// Provides: {"impl_909"}
// Dependencies: {}
# [pyo3 :: pymethods] impl CRLIterator { fn __len__ (& self) -> usize { self . contents . borrow_dependent () . clone () . map_or (0 , | v | v . len ()) } fn __iter__ (slf : pyo3 :: PyRef < '_ , Self >) -> pyo3 :: PyRef < '_ , Self > { slf } fn __next__ (& mut self , py : pyo3 :: Python < '_ >) -> Option < RevokedCertificate > { let revoked = try_map_arc_data_mut_crl_iterator (py , & mut self . contents , | v | match v { Some (v) => match v . next () { Some (revoked) => Ok (revoked) , None => Err (()) , } , None => Err (()) , }) . ok () ? ; Some (RevokedCertificate { owned : revoked , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) } }
};
}
