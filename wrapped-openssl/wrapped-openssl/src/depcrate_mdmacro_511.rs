// Generated macro for macro_511 (macro)
macro_rules! Depcrate_mdmacro_511 {
() => {
// Module: crate::md
// Provides: {"macro_511"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl300)] { use foreign_types :: ForeignType ; use std :: ops :: { Deref , DerefMut } ; type Inner = * mut ffi :: EVP_MD ; impl Drop for Md { # [inline] fn drop (& mut self) { unsafe { ffi :: EVP_MD_free (self . as_ptr ()) ; } } } impl ForeignType for Md { type CType = ffi :: EVP_MD ; type Ref = MdRef ; # [inline] unsafe fn from_ptr (ptr : * mut Self :: CType) -> Self { Md (ptr) } # [inline] fn as_ptr (& self) -> * mut Self :: CType { self . 0 } } impl Deref for Md { type Target = MdRef ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { MdRef :: from_ptr (self . as_ptr ()) } } } impl DerefMut for Md { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { MdRef :: from_ptr_mut (self . as_ptr ()) } } } } else { enum Inner { } } }
};
}
