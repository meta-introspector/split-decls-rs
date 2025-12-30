// Generated macro for impl_517 (impl)
macro_rules! Depcrate_paddingimpl_517 {
() => {
// Module: crate::padding
// Provides: {"impl_517"}
// Dependencies: {}
# [pyo3 :: pymethods] impl ANSIX923PaddingContext { # [new] pub (crate) fn new (block_size : usize) -> ANSIX923PaddingContext { ANSIX923PaddingContext { block_size : block_size / 8 , length_seen : Some (0) , } } pub (crate) fn update < 'a > (& mut self , buf : CffiBuf < 'a > ,) -> CryptographyResult < pyo3 :: Bound < 'a , pyo3 :: PyAny > > { match self . length_seen . as_mut () { Some (v) => { * v += buf . as_bytes () . len () ; Ok (buf . into_pyobj ()) } None => Err (exceptions :: already_finalized_error ()) , } } pub (crate) fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { match self . length_seen . take () { Some (v) => { let pad_size = self . block_size - (v % self . block_size) ; let mut pad = vec ! [0_u8 ; pad_size - 1] ; pad . push (pad_size as u8) ; Ok (pyo3 :: types :: PyBytes :: new (py , pad . as_slice ())) } None => Err (exceptions :: already_finalized_error ()) , } } }
};
}
