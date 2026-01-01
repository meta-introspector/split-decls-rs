/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_USE_0001
/* FP:errors.rs-0002 */ use rustc_macros :: Diagnostic ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0003
/* FP:errors.rs-0006 */ # [derive (Diagnostic)] # [diag (mir_dataflow_path_must_end_in_filename)] pub (crate) struct PathMustEndInFilename { # [primary_span] pub span : Span , }
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0004
/* FP:errors.rs-0008 */ # [derive (Diagnostic)] # [diag (mir_dataflow_unknown_formatter)] pub (crate) struct UnknownFormatter { # [primary_span] pub span : Span , }
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (mir_dataflow_duplicate_values_for)] pub (crate) struct DuplicateValuesFor { # [primary_span] pub span : Span , pub name : Symbol , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0006
/* FP:errors.rs-0012 */ # [derive (Diagnostic)] # [diag (mir_dataflow_requires_an_argument)] pub (crate) struct RequiresAnArgument { # [primary_span] pub span : Span , pub name : Symbol , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (mir_dataflow_stop_after_dataflow_ended_compilation)] pub (crate) struct StopAfterDataFlowEndedCompilation ;
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (mir_dataflow_peek_must_be_place_or_ref_place)] pub (crate) struct PeekMustBePlaceOrRefPlace { # [primary_span] pub span : Span , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (mir_dataflow_peek_must_be_not_temporary)] pub (crate) struct PeekMustBeNotTemporary { # [primary_span] pub span : Span , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (mir_dataflow_peek_bit_not_set)] pub (crate) struct PeekBitNotSet { # [primary_span] pub span : Span , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (mir_dataflow_peek_argument_not_a_local)] pub (crate) struct PeekArgumentNotALocal { # [primary_span] pub span : Span , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (mir_dataflow_peek_argument_untracked)] pub (crate) struct PeekArgumentUntracked { # [primary_span] pub span : Span , }