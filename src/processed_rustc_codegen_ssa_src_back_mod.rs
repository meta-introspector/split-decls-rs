/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_USE_0001
/* FP:mod.rs-0002 */ use std :: borrow :: Cow ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: Session ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0005
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0008
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0010
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_MOD_0012
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_mod_FN_0013
/* FP:mod.rs-0026 */ # [doc = " The target triple depends on the deployment target, and is required to"] # [doc = " enable features such as cross-language LTO, and for picking the right"] # [doc = " Mach-O commands."] # [doc = ""] # [doc = " Certain optimizations also depend on the deployment target."] pub fn versioned_llvm_target (sess : & Session) -> Cow < '_ , str > { if sess . target . is_like_darwin { apple :: add_version_to_llvm_target (& sess . target . llvm_target , sess . apple_deployment_target ()) . into () } else { Cow :: Borrowed (& sess . target . llvm_target) } }