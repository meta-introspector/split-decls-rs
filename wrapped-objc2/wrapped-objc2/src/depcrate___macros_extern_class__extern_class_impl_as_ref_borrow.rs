// Generated macro for __extern_class_impl_as_ref_borrow (macro)
macro_rules! Depcrate___macros_extern_class__extern_class_impl_as_ref_borrow {
() => {
// Module: crate::__macros::extern_class
// Provides: {"__extern_class_impl_as_ref_borrow"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_class_impl_as_ref_borrow { { () ($ ($ attr_impl : tt) *) (impl $ ($ after_impl : tt) *) ($ ($ for : tt) *) fn as_ref ($ ($ self : tt) *) $ as_ref : block } => { } ; { ($ superclass : path $ (, $ remaining_superclasses : path) *) ($ ($ attr_impl : tt) *) (impl $ ($ after_impl : tt) *) ($ ($ for : tt) *) fn as_ref ($ ($ self : tt) *) $ as_ref : block } => { $ ($ attr_impl) * impl $ ($ after_impl) * $ crate :: __macros :: AsRef <$ superclass > for $ ($ for) * { # [inline] fn as_ref ($ ($ self) *) -> &$ superclass $ as_ref } $ ($ attr_impl) * impl $ ($ after_impl) * $ crate :: __macros :: Borrow <$ superclass > for $ ($ for) * { # [inline] fn borrow ($ ($ self) *) -> &$ superclass $ as_ref } $ crate :: __extern_class_impl_as_ref_borrow ! { ($ ($ remaining_superclasses) ,*) ($ ($ attr_impl) *) (impl $ ($ after_impl) *) ($ ($ for) *) fn as_ref ($ ($ self) *) $ as_ref } } ; }
};
}
