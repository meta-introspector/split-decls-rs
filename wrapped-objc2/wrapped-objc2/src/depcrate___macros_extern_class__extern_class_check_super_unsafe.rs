// Generated macro for __extern_class_check_super_unsafe (macro)
macro_rules! Depcrate___macros_extern_class__extern_class_check_super_unsafe {
() => {
// Module: crate::__macros::extern_class
// Provides: {"__extern_class_check_super_unsafe"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_class_check_super_unsafe { (unsafe $ ($ superclass : tt) +) => { } ; (safe $ ($ superclass : tt) +) => { $ crate :: __macros :: compile_error ! ("#[super(...)] must be wrapped in `unsafe`, as in #[unsafe(super(...))]") ; } ; () => { $ crate :: __macros :: compile_error ! ("must specify the superclass with #[unsafe(super(...))]") ; } ; }
};
}
