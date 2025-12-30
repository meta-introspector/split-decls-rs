// Generated macro for impl_521 (impl)
macro_rules! Depcrate_paddingimpl_521 {
() => {
// Module: crate::padding
// Provides: {"impl_521"}
// Dependencies: {}
# [pyo3 :: pymethods] impl ANSIX923UnpaddingContext { # [new] pub (crate) fn new (block_size : usize) -> ANSIX923UnpaddingContext { ANSIX923UnpaddingContext { block_size : block_size / 8 , buffer : Some (Vec :: new ()) , } } pub (crate) fn update < 'p > (& mut self , buf : CffiBuf < 'p > , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { match self . buffer . as_mut () { Some (v) => { v . extend_from_slice (buf . as_bytes ()) ; let finished_blocks = (v . len () / self . block_size) . saturating_sub (1) ; let result_size = finished_blocks * self . block_size ; let result = v . drain (.. result_size) ; Ok (pyo3 :: types :: PyBytes :: new (py , result . as_slice ())) } None => Err (exceptions :: already_finalized_error ()) , } } pub (crate) fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { match self . buffer . take () { Some (v) => { if v . len () != self . block_size { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid padding bytes.") . into () ,) ; } if ! check_ansix923_padding (& v) { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid padding bytes.") . into () ,) ; } let pad_size = * v . last () . unwrap () ; let result = & v [.. v . len () - pad_size as usize] ; Ok (pyo3 :: types :: PyBytes :: new (py , result)) } None => Err (exceptions :: already_finalized_error ()) , } } }
};
}
