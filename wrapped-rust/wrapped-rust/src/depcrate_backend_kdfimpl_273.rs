// Generated macro for impl_273 (impl)
macro_rules! Depcrate_backend_kdfimpl_273 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_273"}
// Dependencies: {}
impl ConcatKdfHash { fn derive_into_buffer (& mut self , py : pyo3 :: Python < '_ > , key_material : & [u8] , output : & mut [u8] ,) -> CryptographyResult < usize > { if self . used { return Err (exceptions :: already_finalized_error ()) ; } self . used = true ; if output . len () != self . length { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("buffer must be {} bytes" , self . length)) ,)) ; } let algorithm_bound = self . algorithm . bind (py) ; let digest_size = algorithm_bound . getattr (pyo3 :: intern ! (py , "digest_size")) ? . extract :: < usize > () ? ; let mut pos = 0usize ; let mut counter = 1u32 ; while pos < self . length { let mut hash_obj = hashes :: Hash :: new (py , algorithm_bound , None) ? ; hash_obj . update_bytes (& counter . to_be_bytes ()) ? ; hash_obj . update_bytes (key_material) ? ; if let Some (ref otherinfo) = self . otherinfo { hash_obj . update_bytes (otherinfo . as_bytes (py)) ? ; } let block = hash_obj . finalize (py) ? ; let block_bytes = block . as_bytes () ; let copy_len = (self . length - pos) . min (digest_size) ; output [pos .. pos + copy_len] . copy_from_slice (& block_bytes [.. copy_len]) ; pos += copy_len ; counter += 1 ; } Ok (self . length) } }
};
}
