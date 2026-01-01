/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_errors_USE_0001
/* FP:errors.rs-0002 */ use rustc_macros :: Diagnostic ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: Span ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_errors_STRUCT_0003
/* FP:errors.rs-0006 */ # [derive (Diagnostic)] # [diag (infer_opaque_hidden_type)] pub (crate) struct OpaqueHiddenTypeDiag { # [primary_span] # [label] pub span : Span , # [note (infer_opaque_type)] pub opaque_type : Span , # [note (infer_hidden_type)] pub hidden_type : Span , }