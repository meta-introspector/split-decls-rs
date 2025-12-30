// Generated macro for impl_520 (impl)
macro_rules! Depcrate_mdimpl_520 {
() => {
// Module: crate::md
// Provides: {"impl_520"}
// Dependencies: {}
impl MdRef { # [doc = " Returns the block size of the digest in bytes."] # [corresponds (EVP_MD_block_size)] # [inline] pub fn block_size (& self) -> usize { unsafe { ffi :: EVP_MD_block_size (self . as_ptr ()) as usize } } # [doc = " Returns the size of the digest in bytes."] # [corresponds (EVP_MD_size)] # [inline] pub fn size (& self) -> usize { unsafe { ffi :: EVP_MD_size (self . as_ptr ()) as usize } } # [doc = " Returns the [`Nid`] of the digest."] # [corresponds (EVP_MD_type)] # [inline] pub fn type_ (& self) -> Nid { unsafe { Nid :: from_raw (ffi :: EVP_MD_type (self . as_ptr ())) } } }
};
}
