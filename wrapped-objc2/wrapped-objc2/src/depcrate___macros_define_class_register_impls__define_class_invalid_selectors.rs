// Generated macro for __define_class_invalid_selectors (macro)
macro_rules! Depcrate___macros_define_class_register_impls__define_class_invalid_selectors {
() => {
// Module: crate::__macros::define_class::register_impls
// Provides: {"__define_class_invalid_selectors"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_invalid_selectors { (method (dealloc)) => { $ crate :: __macros :: compile_error ! ("`#[unsafe(method(dealloc))]` is not supported. Implement `Drop` for the type instead") } ; (method_id (dealloc)) => { $ crate :: __macros :: compile_error ! ("`#[unsafe(method_id(dealloc))]` is not supported. Implement `Drop` for the type instead") } ; (method_id (alloc)) => { $ crate :: __macros :: compile_error ! ($ crate :: __macros :: concat ! ("`#[unsafe(method_id(alloc))]` is not supported. " , "Use `#[unsafe(method(alloc))]` and do the memory management yourself" ,)) } ; (method_id (retain)) => { $ crate :: __macros :: compile_error ! ($ crate :: __macros :: concat ! ("`#[unsafe(method_id(retain))]` is not supported. " , "Use `#[unsafe(method(retain))]` and do the memory management yourself" ,)) } ; (method_id (release)) => { $ crate :: __macros :: compile_error ! ($ crate :: __macros :: concat ! ("`#[unsafe(method_id(release))]` is not supported. " , "Use `#[unsafe(method(release))]` and do the memory management yourself" ,)) } ; (method_id (autorelease)) => { $ crate :: __macros :: compile_error ! ($ crate :: __macros :: concat ! ("`#[unsafe(method_id(autorelease))]` is not supported. " , "Use `#[unsafe(method(autorelease))]` and do the memory management yourself" ,)) } ; ($ method_or_method_id : ident ($ ($ sel : tt) *)) => { } ; }
};
}
