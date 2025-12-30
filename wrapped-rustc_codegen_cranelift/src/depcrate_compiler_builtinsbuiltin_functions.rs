// Generated macro for builtin_functions (macro)
macro_rules! Depcrate_compiler_builtinsbuiltin_functions {
() => {
// Module: crate::compiler_builtins
// Provides: {"builtin_functions"}
// Dependencies: {}
macro_rules ! builtin_functions { ($ register : ident ; $ ($ (# [$ attr : meta]) ? fn $ name : ident ($ ($ arg_name : ident : $ arg_ty : ty) ,*) -> $ ret_ty : ty ;) *) => { # [cfg (feature = "jit")] # [allow (improper_ctypes)] extern "C" { $ ($ (# [$ attr]) ? fn $ name ($ ($ arg_name : $ arg_ty) ,*) -> $ ret_ty ;) * } # [cfg (feature = "jit")] pub (crate) fn $ register (builder : & mut cranelift_jit :: JITBuilder) { for (name , val) in [$ ($ (# [$ attr]) ? (stringify ! ($ name) , $ name as * const u8)) ,*] { builder . symbol (name , val) ; } } } ; }
};
}
