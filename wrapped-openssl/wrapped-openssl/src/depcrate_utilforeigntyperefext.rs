// Generated macro for ForeignTypeRefExt (trait)
macro_rules! Depcrate_utilForeignTypeRefExt {
() => {
// Module: crate::util
// Provides: {"ForeignTypeRefExt"}
// Dependencies: {}
pub trait ForeignTypeRefExt : ForeignTypeRef { unsafe fn from_const_ptr < 'a > (ptr : * const Self :: CType) -> & 'a Self { Self :: from_ptr (ptr as * mut Self :: CType) } unsafe fn from_const_ptr_opt < 'a > (ptr : * const Self :: CType) -> Option < & 'a Self > { if ptr . is_null () { None } else { Some (Self :: from_const_ptr (ptr as * mut Self :: CType)) } } }
};
}
