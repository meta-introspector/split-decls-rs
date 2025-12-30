// Generated macro for ForeignTypeExt (trait)
macro_rules! Depcrate_utilForeignTypeExt {
() => {
// Module: crate::util
// Provides: {"ForeignTypeExt"}
// Dependencies: {}
pub trait ForeignTypeExt : ForeignType { unsafe fn from_ptr_opt (ptr : * mut Self :: CType) -> Option < Self > { if ptr . is_null () { None } else { Some (Self :: from_ptr (ptr)) } } }
};
}
