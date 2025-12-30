// Generated macro for impl_264 (impl)
macro_rules! Depcrate_backend_kdfimpl_264 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_264"}
// Dependencies: {}
impl Hkdf { fn derive_into_buffer (& mut self , py : pyo3 :: Python < '_ > , key_material : & [u8] , output : & mut [u8] ,) -> CryptographyResult < usize > { if self . used { return Err (exceptions :: already_finalized_error ()) ; } self . used = true ; if output . len () != self . length { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("buffer must be {} bytes" , self . length)) ,)) ; } let buf = CffiBuf :: from_bytes (py , key_material) ; let prk = hkdf_extract (py , & self . algorithm , self . salt . as_ref () , & buf) ? ; let mut hkdf_expand = HkdfExpand :: new (py , self . algorithm . clone_ref (py) , self . length , self . info . as_ref () . map (| i | i . clone_ref (py)) , None ,) ? ; hkdf_expand . derive_into_buffer (py , & prk , output) } }
};
}
