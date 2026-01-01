/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_errors_USE_0001
/* FP:errors.rs-0002 */ use rustc_macros :: Diagnostic ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: Span ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_errors_STRUCT_0003
/* FP:errors.rs-0006 */ # [derive (Diagnostic)] # [diag (codegen_gcc_unwinding_inline_asm)] pub (crate) struct UnwindingInlineAsm { # [primary_span] pub span : Span , }
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_errors_STRUCT_0004
/* FP:errors.rs-0008 */ # [derive (Diagnostic)] # [diag (codegen_gcc_copy_bitcode)] pub (crate) struct CopyBitcode { pub err : std :: io :: Error , }
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (codegen_gcc_lto_bitcode_from_rlib)] pub (crate) struct LtoBitcodeFromRlib { pub gcc_err : String , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_errors_STRUCT_0006
/* FP:errors.rs-0012 */ # [derive (Diagnostic)] # [diag (codegen_gcc_explicit_tail_calls_unsupported)] pub (crate) struct ExplicitTailCallsUnsupported ;