// Generated macro for __define_class_register_impls (macro)
macro_rules! Depcrate___macros_define_class_register_impls__define_class_register_impls {
() => {
// Module: crate::__macros::define_class::register_impls
// Provides: {"__define_class_register_impls"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_register_impls { (($ builder : ident)) => { } ; (($ builder : ident) $ (# [$ ($ m : tt) *]) * unsafe impl $ protocol : ident for $ for : ty { $ ($ methods : tt) * } $ ($ rest : tt) *) => { $ crate :: __extract_and_apply_cfg_attributes ! { ($ (# [$ ($ m) *]) *) # [allow (unused_mut)] let mut __objc2_protocol_builder = $ builder . add_protocol_methods ::< dyn $ protocol > () ; # [allow (deprecated)] # [allow (unused_unsafe)] unsafe { $ crate :: __define_class_register_methods ! { (__objc2_protocol_builder) $ ($ methods) * } } __objc2_protocol_builder . finish () ; } $ crate :: __define_class_register_impls ! { ($ builder) $ ($ rest) * } } ; (($ builder : ident) $ (# [$ ($ m : tt) *]) * impl $ for : ty { $ ($ methods : tt) * } $ ($ rest : tt) *) => { $ crate :: __extract_and_apply_cfg_attributes ! { ($ (# [$ ($ m) *]) *) # [allow (deprecated)] # [allow (unused_unsafe)] unsafe { $ crate :: __define_class_register_methods ! { ($ builder) $ ($ methods) * } } } $ crate :: __define_class_register_impls ! { ($ builder) $ ($ rest) * } } ; }
};
}
