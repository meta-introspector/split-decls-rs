// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_x509_verifyimpl_1130 {
() => {
// Module: crate::x509::verify
// Provides: {"impl_1130"}
// Dependencies: {}
impl PolicyBuilder { fn py_clone (& self , py : pyo3 :: Python < '_ >) -> PolicyBuilder { PolicyBuilder { time : self . time . clone () , store : self . store . as_ref () . map (| s | s . clone_ref (py)) , max_chain_depth : self . max_chain_depth , ca_ext_policy : self . ca_ext_policy . as_ref () . map (| p | p . clone_ref (py)) , ee_ext_policy : self . ee_ext_policy . as_ref () . map (| p | p . clone_ref (py)) , } } }
};
}
