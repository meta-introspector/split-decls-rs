/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0001
/* FP:errors.rs-0002 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { Diag , LintDiagnostic } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0003
/* FP:errors.rs-0006 */ use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: mir :: AssertKind ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0005
/* FP:errors.rs-0010 */ use crate :: rustc_complete :: query :: Key ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0007
/* FP:errors.rs-0014 */ use crate :: rustc_complete :: lint :: { self , Lint } ;
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0008
/* FP:errors.rs-0016 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0009
/* FP:errors.rs-0018 */ use crate :: rustc_complete :: { Ident , Span , Symbol } ;
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_USE_0010
/* FP:errors.rs-0020 */ use crate :: fluent_generated as fluent ;
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_FN_0011
/* FP:errors.rs-0022 */ # [doc = " Emit diagnostic for calls to `#[inline(always)]`-annotated functions with a"] # [doc = " `#[target_feature]` attribute where the caller enables a different set of target features."] pub (crate) fn emit_inline_always_target_feature_diagnostic < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , call_span : Span , callee_def_id : DefId , caller_def_id : DefId , callee_only : & [& 'a str] ,) { let callee = tcx . def_path_str (callee_def_id) ; let caller = tcx . def_path_str (caller_def_id) ; tcx . node_span_lint (lint :: builtin :: INLINE_ALWAYS_MISMATCHING_TARGET_FEATURES , tcx . local_def_id_to_hir_id (caller_def_id . as_local () . unwrap ()) , call_span , | lint | { lint . primary_message (format ! ("call to `#[inline(always)]`-annotated `{callee}` \
/* FP:errors.rs-0023 */                 requires the same target features to be inlined")) ; lint . note ("function will not be inlined") ; lint . note (format ! ("the following target features are on `{callee}` but missing from `{caller}`: {}" , callee_only . join (", "))) ; lint . span_note (callee_def_id . default_span (tcx) , format ! ("`{callee}` is defined here")) ; let feats = callee_only . join (",") ; lint . span_suggestion (tcx . def_span (caller_def_id) . shrink_to_lo () , format ! ("add `#[target_feature]` attribute to `{caller}`") , format ! ("#[target_feature(enable = \"{feats}\")]\n") , lint :: Applicability :: MaybeIncorrect ,) ; } ,) ; }
/* FP:errors.rs-0024 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0012
/* FP:errors.rs-0025 */ # [derive (LintDiagnostic)] # [diag (mir_transform_unconditional_recursion)] # [help] pub (crate) struct UnconditionalRecursion { # [label] pub (crate) span : Span , # [label (mir_transform_unconditional_recursion_call_site_label)] pub (crate) call_sites : Vec < Span > , }
/* FP:errors.rs-0026 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0013
/* FP:errors.rs-0027 */ # [derive (Diagnostic)] # [diag (mir_transform_force_inline_attr)] # [note] pub (crate) struct InvalidForceInline { # [primary_span] pub attr_span : Span , # [label (mir_transform_callee)] pub callee_span : Span , pub callee : String , pub reason : & 'static str , }
/* FP:errors.rs-0028 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_ENUM_0014
/* FP:errors.rs-0029 */ # [derive (LintDiagnostic)] pub (crate) enum ConstMutate { # [diag (mir_transform_const_modify)] # [note] Modify { # [note (mir_transform_const_defined_here)] konst : Span , } , # [diag (mir_transform_const_mut_borrow)] # [note] # [note (mir_transform_note2)] MutBorrow { # [note (mir_transform_note3)] method_call : Option < Span > , # [note (mir_transform_const_defined_here)] konst : Span , } , }
/* FP:errors.rs-0030 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0015
/* FP:errors.rs-0031 */ # [derive (Diagnostic)] # [diag (mir_transform_unaligned_packed_ref , code = E0793)] # [note] # [note (mir_transform_note_ub)] # [help] pub (crate) struct UnalignedPackedRef { # [primary_span] pub span : Span , }
/* FP:errors.rs-0032 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0016
/* FP:errors.rs-0033 */ # [derive (Diagnostic)] # [diag (mir_transform_unknown_pass_name)] pub (crate) struct UnknownPassName < 'a > { pub (crate) name : & 'a str , }
/* FP:errors.rs-0034 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0017
/* FP:errors.rs-0035 */ pub (crate) struct AssertLint < P > { pub span : Span , pub assert_kind : AssertKind < P > , pub lint_kind : AssertLintKind , }
/* FP:errors.rs-0036 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_ENUM_0018
/* FP:errors.rs-0037 */ pub (crate) enum AssertLintKind { ArithmeticOverflow , UnconditionalPanic , }
/* FP:errors.rs-0038 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_IMPL_0019
/* FP:errors.rs-0039 */ impl < 'a , P : std :: fmt :: Debug > LintDiagnostic < 'a , () > for AssertLint < P > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (match self . lint_kind { AssertLintKind :: ArithmeticOverflow => fluent :: mir_transform_arithmetic_overflow , AssertLintKind :: UnconditionalPanic => fluent :: mir_transform_operation_will_panic , }) ; let label = self . assert_kind . diagnostic_message () ; self . assert_kind . add_args (& mut | name , value | { diag . arg (name , value) ; }) ; diag . span_label (self . span , label) ; } }
/* FP:errors.rs-0040 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_IMPL_0020
/* FP:errors.rs-0041 */ impl AssertLintKind { pub (crate) fn lint (& self) -> & 'static Lint { match self { AssertLintKind :: ArithmeticOverflow => lint :: builtin :: ARITHMETIC_OVERFLOW , AssertLintKind :: UnconditionalPanic => lint :: builtin :: UNCONDITIONAL_PANIC , } } }
/* FP:errors.rs-0042 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0021
/* FP:errors.rs-0043 */ # [derive (LintDiagnostic)] # [diag (mir_transform_ffi_unwind_call)] pub (crate) struct FfiUnwindCall { # [label (mir_transform_ffi_unwind_call)] pub span : Span , pub foreign : bool , }
/* FP:errors.rs-0044 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0022
/* FP:errors.rs-0045 */ # [derive (LintDiagnostic)] # [diag (mir_transform_fn_item_ref)] pub (crate) struct FnItemRef { # [suggestion (code = "{sugg}" , applicability = "unspecified")] pub span : Span , pub sugg : String , pub ident : Ident , }
/* FP:errors.rs-0046 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0023
/* FP:errors.rs-0047 */ pub (crate) struct MustNotSupend < 'a , 'tcx > { pub tcx : TyCtxt < 'tcx > , pub yield_sp : Span , pub reason : Option < MustNotSuspendReason > , pub src_sp : Span , pub pre : & 'a str , pub def_id : DefId , pub post : & 'a str , }
/* FP:errors.rs-0048 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_IMPL_0024
/* FP:errors.rs-0049 */ impl < 'a > LintDiagnostic < 'a , () > for MustNotSupend < '_ , '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut crate :: rustc_errors :: Diag < 'a , () >) { diag . primary_message (fluent :: mir_transform_must_not_suspend) ; diag . span_label (self . yield_sp , fluent :: _subdiag :: label) ; if let Some (reason) = self . reason { diag . subdiagnostic (reason) ; } diag . span_help (self . src_sp , fluent :: _subdiag :: help) ; diag . arg ("pre" , self . pre) ; diag . arg ("def_path" , self . tcx . def_path_str (self . def_id)) ; diag . arg ("post" , self . post) ; } }
/* FP:errors.rs-0050 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0025
/* FP:errors.rs-0051 */ # [derive (Subdiagnostic)] # [note (mir_transform_note)] pub (crate) struct MustNotSuspendReason { # [primary_span] pub span : Span , pub reason : String , }
/* FP:errors.rs-0052 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0026
/* FP:errors.rs-0053 */ # [derive (Diagnostic)] # [diag (mir_transform_force_inline)] # [note] pub (crate) struct ForceInlineFailure { # [label (mir_transform_caller)] pub caller_span : Span , # [label (mir_transform_callee)] pub callee_span : Span , # [label (mir_transform_attr)] pub attr_span : Span , # [primary_span] # [label (mir_transform_call)] pub call_span : Span , pub callee : String , pub caller : String , pub reason : & 'static str , # [subdiagnostic] pub justification : Option < ForceInlineJustification > , }
/* FP:errors.rs-0054 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_errors_STRUCT_0027
/* FP:errors.rs-0055 */ # [derive (Subdiagnostic)] # [note (mir_transform_force_inline_justification)] pub (crate) struct ForceInlineJustification { pub sym : Symbol , }