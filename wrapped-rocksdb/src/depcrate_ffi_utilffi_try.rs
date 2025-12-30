// Generated macro for ffi_try (macro)
macro_rules! Depcrate_ffi_utilffi_try {
() => {
// Module: crate::ffi_util
// Provides: {"ffi_try"}
// Dependencies: {}
macro_rules ! ffi_try { ($ ($ function : ident) ::* ()) => { ffi_try_impl ! ($ ($ function) ::* ()) } ; ($ ($ function : ident) ::* ($ arg1 : expr $ (, $ arg : expr) * $ (,) ?)) => { ffi_try_impl ! ($ ($ function) ::* ($ arg1 $ (, $ arg) * ,)) } ; }
};
}
