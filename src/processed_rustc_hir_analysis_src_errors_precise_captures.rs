/* FP:precise_captures.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_USE_0001
/* FP:precise_captures.rs-0002 */ use crate :: rustc_complete :: E0799 ;
/* FP:precise_captures.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_USE_0002
/* FP:precise_captures.rs-0004 */ use rustc_macros :: Diagnostic ;
/* FP:precise_captures.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_USE_0003
/* FP:precise_captures.rs-0006 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:precise_captures.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0004
/* FP:precise_captures.rs-0008 */ # [derive (Diagnostic)] # [diag (hir_analysis_param_not_captured)] # [note] pub (crate) struct ParamNotCaptured { # [primary_span] pub opaque_span : Span , # [label] pub param_span : Span , pub kind : & 'static str , }
/* FP:precise_captures.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0005
/* FP:precise_captures.rs-0010 */ # [derive (Diagnostic)] # [diag (hir_analysis_self_ty_not_captured)] # [note] pub (crate) struct SelfTyNotCaptured { # [primary_span] pub opaque_span : Span , # [label] pub trait_span : Span , }
/* FP:precise_captures.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0006
/* FP:precise_captures.rs-0012 */ # [derive (Diagnostic)] # [diag (hir_analysis_lifetime_not_captured)] pub (crate) struct LifetimeNotCaptured { # [primary_span] pub use_span : Span , # [label (hir_analysis_param_label)] pub param_span : Span , # [label] pub opaque_span : Span , }
/* FP:precise_captures.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0007
/* FP:precise_captures.rs-0014 */ # [derive (Diagnostic)] # [diag (hir_analysis_lifetime_implicitly_captured)] pub (crate) struct LifetimeImplicitlyCaptured { # [primary_span] pub opaque_span : Span , # [label (hir_analysis_param_label)] pub param_span : Span , }
/* FP:precise_captures.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0008
/* FP:precise_captures.rs-0016 */ # [derive (Diagnostic)] # [diag (hir_analysis_bad_precise_capture)] pub (crate) struct BadPreciseCapture { # [primary_span] pub span : Span , pub kind : & 'static str , pub found : String , }
/* FP:precise_captures.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0009
/* FP:precise_captures.rs-0018 */ # [derive (Diagnostic)] # [diag (hir_analysis_precise_capture_self_alias , code = E0799)] pub (crate) struct PreciseCaptureSelfAlias { # [primary_span] pub span : Span , # [label] pub self_span : Span , pub what : & 'static str , }
/* FP:precise_captures.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0010
/* FP:precise_captures.rs-0020 */ # [derive (Diagnostic)] # [diag (hir_analysis_duplicate_precise_capture)] pub (crate) struct DuplicatePreciseCapture { # [primary_span] pub first_span : Span , pub name : Symbol , # [label] pub second_span : Span , }
/* FP:precise_captures.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_errors_precise_captures_STRUCT_0011
/* FP:precise_captures.rs-0022 */ # [derive (Diagnostic)] # [diag (hir_analysis_lifetime_must_be_first)] pub (crate) struct LifetimesMustBeFirst { # [primary_span] pub lifetime_span : Span , pub name : Symbol , # [label] pub other_span : Span , }