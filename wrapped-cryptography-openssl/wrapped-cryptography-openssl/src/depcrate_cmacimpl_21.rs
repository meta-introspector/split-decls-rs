// Generated macro for impl_21 (impl)
macro_rules! Depcrate_cmacimpl_21 {
() => {
// Module: crate::cmac
// Provides: {"impl_21"}
// Dependencies: {}
impl CmacRef { pub fn update (& mut self , data : & [u8]) -> OpenSSLResult < () > { unsafe { cvt (ffi :: CMAC_Update (self . as_ptr () , data . as_ptr () . cast () , data . len () ,)) ? ; } Ok (()) } pub fn finish (& mut self) -> OpenSSLResult < DigestBytes > { let mut buf = [0 ; ffi :: EVP_MAX_MD_SIZE as usize] ; let mut len = ffi :: EVP_MAX_MD_SIZE as usize ; unsafe { cvt (ffi :: CMAC_Final (self . as_ptr () , buf . as_mut_ptr () , & mut len)) ? ; } Ok (DigestBytes { buf , len }) } pub fn copy (& self) -> OpenSSLResult < Cmac > { unsafe { let h = Cmac :: from_ptr (cvt_p (ffi :: CMAC_CTX_new ()) ?) ; cvt (ffi :: CMAC_CTX_copy (h . as_ptr () , self . as_ptr ())) ? ; Ok (h) } } }
};
}
