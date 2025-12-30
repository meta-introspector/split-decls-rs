// Generated macro for impl_267 (impl)
macro_rules! Depcrate_backend_kdfimpl_267 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_267"}
// Dependencies: {}
impl HkdfExpand { fn derive_into_buffer (& mut self , py : pyo3 :: Python < '_ > , key_material : & [u8] , output : & mut [u8] ,) -> CryptographyResult < usize > { if self . used { return Err (exceptions :: already_finalized_error ()) ; } self . used = true ; if output . len () != self . length { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("buffer must be {} bytes" , self . length)) ,)) ; } let algorithm_bound = self . algorithm . bind (py) ; let h_prime = Hmac :: new_bytes (py , key_material , algorithm_bound) ? ; let digest_size = algorithm_bound . getattr (pyo3 :: intern ! (py , "digest_size")) ? . extract :: < usize > () ? ; let mut pos = 0usize ; let mut counter = 0u8 ; while pos < self . length { counter += 1 ; let mut h = h_prime . copy (py) ? ; let start = pos . saturating_sub (digest_size) ; h . update_bytes (& output [start .. pos]) ? ; h . update_bytes (self . info . as_bytes (py)) ? ; h . update_bytes (& [counter]) ? ; let block = h . finalize (py) ? ; let block_bytes = block . as_bytes () ; let copy_len = (self . length - pos) . min (digest_size) ; output [pos .. pos + copy_len] . copy_from_slice (& block_bytes [.. copy_len]) ; pos += copy_len ; } Ok (self . length) } }
};
}
