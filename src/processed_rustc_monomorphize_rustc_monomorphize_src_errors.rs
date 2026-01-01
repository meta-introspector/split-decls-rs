/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_USE_0001
/* FP:errors.rs-0002 */ use rustc_macros :: { Diagnostic , LintDiagnostic } ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: ty :: { Instance , Ty } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_USE_0003
/* FP:errors.rs-0006 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0004
/* FP:errors.rs-0008 */ # [derive (Diagnostic)] # [diag (monomorphize_recursion_limit)] pub (crate) struct RecursionLimit < 'tcx > { # [primary_span] pub span : Span , pub instance : Instance < 'tcx > , # [note] pub def_span : Span , pub def_path_str : String , }
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (monomorphize_no_optimized_mir)] pub (crate) struct NoOptimizedMir { # [note] pub span : Span , pub crate_name : Symbol , pub instance : String , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0006
/* FP:errors.rs-0012 */ # [derive (LintDiagnostic)] # [diag (monomorphize_large_assignments)] # [note] pub (crate) struct LargeAssignmentsLint { # [label] pub span : Span , pub size : u64 , pub limit : u64 , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (monomorphize_symbol_already_defined)] pub (crate) struct SymbolAlreadyDefined { # [primary_span] pub span : Option < Span > , pub symbol : String , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (monomorphize_couldnt_dump_mono_stats)] pub (crate) struct CouldntDumpMonoStats { pub error : String , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (monomorphize_encountered_error_while_instantiating)] pub (crate) struct EncounteredErrorWhileInstantiating < 'tcx > { # [primary_span] pub span : Span , pub kind : & 'static str , pub instance : Instance < 'tcx > , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (monomorphize_encountered_error_while_instantiating_global_asm)] pub (crate) struct EncounteredErrorWhileInstantiatingGlobalAsm { # [primary_span] pub span : Span , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (Diagnostic)] # [diag (monomorphize_start_not_found)] # [help] pub (crate) struct StartNotFound ;
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (monomorphize_abi_error_disabled_vector_type)] # [help] pub (crate) struct AbiErrorDisabledVectorType < 'a > { # [primary_span] # [label] pub span : Span , pub required_feature : & 'a str , pub ty : Ty < 'a > , # [doc = " Whether this is a problem at a call site or at a declaration."] pub is_call : bool , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (monomorphize_abi_error_unsupported_vector_type)] pub (crate) struct AbiErrorUnsupportedVectorType < 'a > { # [primary_span] # [label] pub span : Span , pub ty : Ty < 'a > , # [doc = " Whether this is a problem at a call site or at a declaration."] pub is_call : bool , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Diagnostic)] # [diag (monomorphize_abi_required_target_feature)] # [help] pub (crate) struct AbiRequiredTargetFeature < 'a > { # [primary_span] # [label] pub span : Span , pub required_feature : & 'a str , pub abi : & 'a str , # [doc = " Whether this is a problem at a call site or at a declaration."] pub is_call : bool , }