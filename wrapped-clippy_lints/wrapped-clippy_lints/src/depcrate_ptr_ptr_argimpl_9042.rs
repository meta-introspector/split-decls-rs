// Generated macro for impl_9042 (impl)
macro_rules! Depcrate_ptr_ptr_argimpl_9042 {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"impl_9042"}
// Dependencies: {}
impl fmt :: Display for DerefTyDisplay < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use std :: fmt :: Write ; match self . 1 { DerefTy :: Str => f . write_str ("str") , DerefTy :: Path => f . write_str ("Path") , DerefTy :: Slice (hir_ty , ty) => { f . write_char ('[') ? ; match hir_ty . and_then (| s | s . get_source_text (self . 0)) { Some (s) => f . write_str (& s) ? , None => ty . fmt (f) ? , } f . write_char (']') } , } } }
};
}
