// Generated macro for impl_276 (impl)
macro_rules! Depcrate_backend_kdfimpl_276 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_276"}
// Dependencies: {}
impl ConcatKdfHmac { fn derive_into_buffer (& mut self , py : pyo3 :: Python < '_ > , key_material : & [u8] , output : & mut [u8] ,) -> CryptographyResult < usize > { if self . used { return Err (exceptions :: already_finalized_error ()) ; } self . used = true ; if output . len () != self . length { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("buffer must be {} bytes" , self . length)) ,)) ; } let algorithm_bound = self . algorithm . bind (py) ; let digest_size = algorithm_bound . getattr (pyo3 :: intern ! (py , "digest_size")) ? . extract :: < usize > () ? ; let mut pos = 0usize ; let mut counter = 1u32 ; while pos < self . length { let mut hmac = Hmac :: new_bytes (py , self . salt . as_bytes (py) , algorithm_bound) ? ; hmac . update_bytes (& counter . to_be_bytes ()) ? ; hmac . update_bytes (key_material) ? ; if let Some (ref otherinfo) = self . otherinfo { hmac . update_bytes (otherinfo . as_bytes (py)) ? ; } let result = hmac . finalize_bytes () ? ; let copy_len = (self . length - pos) . min (digest_size) ; output [pos .. pos + copy_len] . copy_from_slice (& result [.. copy_len]) ; pos += copy_len ; counter += 1 ; } Ok (self . length) } }
};
}
