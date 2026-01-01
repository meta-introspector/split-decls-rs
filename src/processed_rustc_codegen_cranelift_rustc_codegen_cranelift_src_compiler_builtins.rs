/* FP:compiler_builtins.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_compiler_builtins_USE_0001
/* FP:compiler_builtins.rs-0002 */ # [cfg (all (unix , feature = "jit"))] use std :: ffi :: c_int ;
/* FP:compiler_builtins.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_compiler_builtins_USE_0002
/* FP:compiler_builtins.rs-0004 */ # [cfg (feature = "jit")] use std :: ffi :: c_void ;
/* FP:compiler_builtins.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_compiler_builtins_TYPE_0003
/* FP:compiler_builtins.rs-0006 */ # [allow (non_camel_case_types)] # [cfg (feature = "jit")] type size_t = usize ;
/* FP:compiler_builtins.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_compiler_builtins_MACRO_0004
/* FP:compiler_builtins.rs-0008 */ macro_rules ! builtin_functions { ($ register : ident ; $ ($ (# [$ attr : meta]) ? fn $ name : ident ($ ($ arg_name : ident : $ arg_ty : ty) ,*) -> $ ret_ty : ty ;) *) => { # [cfg (feature = "jit")] # [allow (improper_ctypes)] extern "C" { $ ($ (# [$ attr]) ? fn $ name ($ ($ arg_name : $ arg_ty) ,*) -> $ ret_ty ;) * } # [cfg (feature = "jit")] pub (crate) fn $ register (builder : & mut cranelift_jit :: JITBuilder) { for (name , val) in [$ ($ (# [$ attr]) ? (stringify ! ($ name) , $ name as * const u8)) ,*] { builder . symbol (name , val) ; } } } ; }
/* FP:compiler_builtins.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_compiler_builtins_MACRO_0005