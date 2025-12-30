// Generated macro for impl_80 (impl)
macro_rules! Depcrate_backend_ciphersimpl_80 {
() => {
// Module: crate::backend::ciphers
// Provides: {"impl_80"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PyCipherContext { fn update < 'p > (& mut self , py : pyo3 :: Python < 'p > , data : CffiBuf < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { get_mut_ctx (self . ctx . as_mut ()) ? . update (py , data . as_bytes ()) } fn reset_nonce (& mut self , py : pyo3 :: Python < '_ > , nonce : CffiBuf < '_ >) -> CryptographyResult < () > { get_mut_ctx (self . ctx . as_mut ()) ? . reset_nonce (py , nonce) } fn update_into (& mut self , py : pyo3 :: Python < '_ > , data : CffiBuf < '_ > , mut buf : CffiMutBuf < '_ > ,) -> CryptographyResult < usize > { get_mut_ctx (self . ctx . as_mut ()) ? . update_into (py , data . as_bytes () , buf . as_mut_bytes ()) } fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let result = get_mut_ctx (self . ctx . as_mut ()) ? . finalize (py) ? ; self . ctx = None ; Ok (result) } }
};
}
