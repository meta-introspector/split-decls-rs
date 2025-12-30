// Generated macro for impl_515 (impl)
macro_rules! Depcrate_paddingimpl_515 {
() => {
// Module: crate::padding
// Provides: {"impl_515"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PKCS7PaddingContext { # [new] pub (crate) fn new (block_size : usize) -> PKCS7PaddingContext { PKCS7PaddingContext { block_size : block_size / 8 , length_seen : Some (0) , } } pub (crate) fn update < 'a > (& mut self , buf : CffiBuf < 'a > ,) -> CryptographyResult < pyo3 :: Bound < 'a , pyo3 :: PyAny > > { match self . length_seen . as_mut () { Some (v) => { * v += buf . as_bytes () . len () ; Ok (buf . into_pyobj ()) } None => Err (exceptions :: already_finalized_error ()) , } } pub (crate) fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { match self . length_seen . take () { Some (v) => { let pad_size = self . block_size - (v % self . block_size) ; let pad = vec ! [pad_size as u8 ; pad_size] ; Ok (pyo3 :: types :: PyBytes :: new (py , & pad)) } None => Err (exceptions :: already_finalized_error ()) , } } }
};
}
