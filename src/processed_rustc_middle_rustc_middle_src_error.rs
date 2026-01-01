/* FP:error.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0001
/* FP:error.rs-0002 */ use std :: path :: Path ;
/* FP:error.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0002
/* FP:error.rs-0004 */ use std :: { fmt , io } ;
/* FP:error.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0003
/* FP:error.rs-0006 */ use crate :: rustc_complete :: codes :: * ;
/* FP:error.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0004
/* FP:error.rs-0008 */ use crate :: rustc_complete :: { DiagArgName , DiagArgValue , DiagMessage } ;
/* FP:error.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0005
/* FP:error.rs-0010 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:error.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0006
/* FP:error.rs-0012 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:error.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_USE_0007
/* FP:error.rs-0014 */ use crate :: ty :: { Instance , Ty } ;
/* FP:error.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0008
/* FP:error.rs-0016 */ # [derive (Diagnostic)] # [diag (middle_drop_check_overflow , code = E0320)] # [note] pub (crate) struct DropCheckOverflow < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub overflow_ty : Ty < 'tcx > , }
/* FP:error.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0009
/* FP:error.rs-0018 */ # [derive (Diagnostic)] # [diag (middle_failed_writing_file)] pub (crate) struct FailedWritingFile < 'a > { pub path : & 'a Path , pub error : io :: Error , }
/* FP:error.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0010
/* FP:error.rs-0020 */ # [derive (Diagnostic)] # [diag (middle_opaque_hidden_type_mismatch)] pub (crate) struct OpaqueHiddenTypeMismatch < 'tcx > { pub self_ty : Ty < 'tcx > , pub other_ty : Ty < 'tcx > , # [primary_span] # [label] pub other_span : Span , # [subdiagnostic] pub sub : TypeMismatchReason , }
/* FP:error.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0011
/* FP:error.rs-0022 */ # [derive (Diagnostic)] # [diag (middle_unsupported_union)] pub struct UnsupportedUnion { pub ty_name : String , }
/* FP:error.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0012
/* FP:error.rs-0024 */ # [derive (Diagnostic)] # [diag (middle_autodiff_unsafe_inner_const_ref)] pub struct AutodiffUnsafeInnerConstRef < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , }
/* FP:error.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_ENUM_0013
/* FP:error.rs-0026 */ # [derive (Subdiagnostic)] pub enum TypeMismatchReason { # [label (middle_conflict_types)] ConflictType { # [primary_span] span : Span , } , # [note (middle_previous_use_here)] PreviousUse { # [primary_span] span : Span , } , }
/* FP:error.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0014
/* FP:error.rs-0028 */ # [derive (Diagnostic)] # [diag (middle_recursion_limit_reached)] # [help] pub (crate) struct RecursionLimitReached < 'tcx > { pub ty : Ty < 'tcx > , pub suggested_limit : crate :: rustc_hir :: limit :: Limit , }
/* FP:error.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0015
/* FP:error.rs-0030 */ # [derive (Diagnostic)] # [diag (middle_const_eval_non_int)] pub (crate) struct ConstEvalNonIntError { # [primary_span] pub span : Span , }
/* FP:error.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0016
/* FP:error.rs-0032 */ # [derive (Diagnostic)] # [diag (middle_strict_coherence_needs_negative_coherence)] pub (crate) struct StrictCoherenceNeedsNegativeCoherence { # [primary_span] pub span : Span , # [label] pub attr_span : Option < Span > , }
/* FP:error.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0017
/* FP:error.rs-0034 */ # [derive (Diagnostic)] # [diag (middle_requires_lang_item)] pub (crate) struct RequiresLangItem { # [primary_span] pub span : Span , pub name : Symbol , }
/* FP:error.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0018
/* FP:error.rs-0036 */ # [derive (Diagnostic)] # [diag (middle_const_not_used_in_type_alias)] pub (super) struct ConstNotUsedTraitAlias { pub ct : String , # [primary_span] pub span : Span , }
/* FP:error.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0019
/* FP:error.rs-0038 */ pub struct CustomSubdiagnostic < 'a > { pub msg : fn () -> DiagMessage , pub add_args : Box < dyn FnOnce (& mut dyn FnMut (DiagArgName , DiagArgValue)) + 'a > , }
/* FP:error.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_IMPL_0020
/* FP:error.rs-0040 */ impl < 'a > CustomSubdiagnostic < 'a > { pub fn label (x : fn () -> DiagMessage) -> Self { Self :: label_and_then (x , | _ | { }) } pub fn label_and_then < F : FnOnce (& mut dyn FnMut (DiagArgName , DiagArgValue)) + 'a > (msg : fn () -> DiagMessage , f : F ,) -> Self { Self { msg , add_args : Box :: new (move | x | f (x)) } } }
/* FP:error.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_IMPL_0021
/* FP:error.rs-0042 */ impl fmt :: Debug for CustomSubdiagnostic < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CustomSubdiagnostic") . finish_non_exhaustive () } }
/* FP:error.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_ENUM_0022
/* FP:error.rs-0044 */ # [derive (Diagnostic)] pub enum LayoutError < 'tcx > { # [diag (middle_layout_unknown)] Unknown { ty : Ty < 'tcx > } , # [diag (middle_layout_too_generic)] TooGeneric { ty : Ty < 'tcx > } , # [diag (middle_layout_size_overflow)] Overflow { ty : Ty < 'tcx > } , # [diag (middle_layout_normalization_failure)] NormalizationFailure { ty : Ty < 'tcx > , failure_ty : String } , # [diag (middle_layout_cycle)] Cycle , # [diag (middle_layout_references_error)] ReferencesError , }
/* FP:error.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0023
/* FP:error.rs-0046 */ # [derive (Diagnostic)] # [diag (middle_erroneous_constant)] pub (crate) struct ErroneousConstant { # [primary_span] pub span : Span , }
/* FP:error.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0024
/* FP:error.rs-0048 */ # [derive (Diagnostic)] # [diag (middle_type_length_limit)] # [help (middle_consider_type_length_limit)] pub (crate) struct TypeLengthLimit < 'tcx > { # [primary_span] pub span : Span , pub instance : Instance < 'tcx > , pub type_length : usize , }
/* FP:error.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0025
/* FP:error.rs-0050 */ # [derive (Diagnostic)] # [diag (middle_max_num_nodes_in_valtree)] pub (crate) struct MaxNumNodesInValtree { # [primary_span] pub span : Span , pub global_const_id : String , }
/* FP:error.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_error_STRUCT_0026
/* FP:error.rs-0052 */ # [derive (Diagnostic)] # [diag (middle_invalid_const_in_valtree)] # [note] pub (crate) struct InvalidConstInValtree { # [primary_span] pub span : Span , pub global_const_id : String , }