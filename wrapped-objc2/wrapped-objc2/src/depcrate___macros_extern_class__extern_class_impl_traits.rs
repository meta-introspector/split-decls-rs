// Generated macro for __extern_class_impl_traits (macro)
macro_rules! Depcrate___macros_extern_class__extern_class_impl_traits {
() => {
// Module: crate::__macros::extern_class
// Provides: {"__extern_class_impl_traits"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_class_impl_traits { (($ ($ attr_impl : tt) *) (unsafe impl $ ($ after_impl : tt) *) ($ ($ for : tt) *) ($ superclass : path $ (, $ remaining_superclasses : path) *)) => { $ ($ attr_impl) * unsafe impl $ ($ after_impl) * $ crate :: RefEncode for $ ($ for) * { const ENCODING_REF : $ crate :: Encoding = <$ superclass as $ crate :: RefEncode >:: ENCODING_REF ; } $ ($ attr_impl) * unsafe impl $ ($ after_impl) * $ crate :: Message for $ ($ for) * { } $ ($ attr_impl) * impl $ ($ after_impl) * $ crate :: __macros :: Deref for $ ($ for) * { type Target = $ superclass ; # [inline] fn deref (& self) -> & Self :: Target { & self . __superclass } } $ ($ attr_impl) * impl $ ($ after_impl) * $ crate :: __macros :: AsRef < Self > for $ ($ for) * { # [inline] fn as_ref (& self) -> & Self { self } } $ crate :: __extern_class_impl_as_ref_borrow ! { ($ superclass $ (, $ remaining_superclasses) *) ($ ($ attr_impl) *) (impl $ ($ after_impl) *) ($ ($ for) *) fn as_ref (& self) { &* self } } } ; }
};
}
