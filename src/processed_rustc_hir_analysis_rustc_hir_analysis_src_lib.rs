/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0007
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0009
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0010
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0011
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0012
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MOD_0013
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0014
/* FP:lib.rs-0028 */ pub use errors :: NoVariantNamed ;
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0015
/* FP:lib.rs-0030 */ use crate :: rustc_abi :: { CVariadicStatus , ExternAbi } ;
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0016
/* FP:lib.rs-0032 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0017
/* FP:lib.rs-0034 */ use crate :: rustc_complete :: lints :: DelayedLint ;
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0018
/* FP:lib.rs-0036 */ use crate :: rustc_complete :: { self as hir } ;
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0019
/* FP:lib.rs-0038 */ use crate :: rustc_complete :: middle ;
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0020
/* FP:lib.rs-0040 */ use crate :: rustc_complete :: mir :: interpret :: GlobalId ;
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0021
/* FP:lib.rs-0042 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0022
/* FP:lib.rs-0044 */ use crate :: rustc_complete :: ty :: { self , Const , Ty , TyCtxt } ;
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0023
/* FP:lib.rs-0046 */ use crate :: rustc_complete :: parse :: feature_err ;
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0024
/* FP:lib.rs-0048 */ use crate :: rustc_complete :: { ErrorGuaranteed , Span } ;
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0025
/* FP:lib.rs-0050 */ use crate :: rustc_trait_selection :: traits ;
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0026
/* FP:lib.rs-0052 */ pub use crate :: collect :: suggest_impl_trait ;
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_USE_0027
/* FP:lib.rs-0054 */ use crate :: hir_ty_lowering :: { FeedConstTy , HirTyLowerer } ;
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_MACRO_0028
/* FP:lib.rs-0056 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_FN_0029
/* FP:lib.rs-0058 */ fn check_c_variadic_abi (tcx : TyCtxt < '_ > , decl : & hir :: FnDecl < '_ > , abi : ExternAbi , span : Span) { if ! decl . c_variadic { return ; } match abi . supports_c_variadic () { CVariadicStatus :: Stable => { } CVariadicStatus :: NotSupported => { tcx . dcx () . create_err (errors :: VariadicFunctionCompatibleConvention { span , convention : & format ! ("{abi}") , }) . emit () ; } CVariadicStatus :: Unstable { feature } => { if ! tcx . features () . enabled (feature) { feature_err (& tcx . sess , feature , span , format ! ("C-variadic functions with the {abi} calling convention are unstable") ,) . emit () ; } } } }
/* FP:lib.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_FN_0030
/* FP:lib.rs-0060 */ # [doc = " Adds query implementations to the [Providers] vtable, see [`crate::rustc_middle::query`]"] pub fn provide (providers : & mut Providers) { collect :: provide (providers) ; coherence :: provide (providers) ; check :: provide (providers) ; * providers = Providers { check_unused_traits : check_unused :: check_unused_traits , diagnostic_hir_wf_check : hir_wf_check :: diagnostic_hir_wf_check , inferred_outlives_crate : outlives :: inferred_outlives_crate , inferred_outlives_of : outlives :: inferred_outlives_of , inherit_sig_for_delegation_item : delegation :: inherit_sig_for_delegation_item , enforce_impl_non_lifetime_params_are_constrained : impl_wf_check :: enforce_impl_non_lifetime_params_are_constrained , crate_variances : variance :: crate_variances , variances_of : variance :: variances_of , .. * providers } ; }
/* FP:lib.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_FN_0031
/* FP:lib.rs-0062 */ fn emit_delayed_lint (lint : & DelayedLint , tcx : TyCtxt < '_ >) { match lint { DelayedLint :: AttributeParsing (attribute_lint) => { rustc_attr_parsing :: emit_attribute_lint (attribute_lint , tcx) } } }
/* FP:lib.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_FN_0032
/* FP:lib.rs-0064 */ pub fn check_crate (tcx : TyCtxt < '_ >) { let _prof_timer = tcx . sess . timer ("type_check_crate") ; tcx . sess . time ("coherence_checking" , | | { type R = Result < () , ErrorGuaranteed > ; let _ : R = tcx . ensure_ok () . check_type_wf (()) ; for & trait_def_id in tcx . all_local_trait_impls (()) . keys () { let _ : R = tcx . ensure_ok () . coherent_trait (trait_def_id) ; } let _ : R = tcx . ensure_ok () . crate_inherent_impls_validity_check (()) ; let _ : R = tcx . ensure_ok () . crate_inherent_impls_overlap_check (()) ; }) ; tcx . sess . time ("emit_ast_lowering_delayed_lints" , | | { # [cfg (debug_assertions)] { for owner_id in tcx . hir_crate_items (()) . owners () { if let Some (delayed_lints) = tcx . opt_ast_lowering_delayed_lints (owner_id) { if ! delayed_lints . lints . is_empty () { assert ! (tcx . hir_crate_items (()) . delayed_lint_items () . any (| i | i == owner_id)) ; } } } } for owner_id in tcx . hir_crate_items (()) . delayed_lint_items () { if let Some (delayed_lints) = tcx . opt_ast_lowering_delayed_lints (owner_id) { for lint in & delayed_lints . lints { emit_delayed_lint (lint , tcx) ; } } } }) ; tcx . par_hir_body_owners (| item_def_id | { let def_kind = tcx . def_kind (item_def_id) ; match def_kind { DefKind :: Static { .. } => { tcx . ensure_ok () . eval_static_initializer (item_def_id) ; check :: maybe_check_static_with_link_section (tcx , item_def_id) ; } DefKind :: Const if ! tcx . generics_of (item_def_id) . own_requires_monomorphization () => { let instance = ty :: Instance :: new_raw (item_def_id . into () , ty :: GenericArgs :: empty ()) ; let cid = GlobalId { instance , promoted : None } ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; tcx . ensure_ok () . eval_to_const_value_raw (typing_env . as_query_input (cid)) ; } _ => () , } if ! (matches ! (def_kind , DefKind :: AnonConst) || def_kind . is_typeck_child ()) { tcx . ensure_ok () . typeck (item_def_id) ; } if tcx . needs_coroutine_by_move_body_def_id (item_def_id . to_def_id ()) { tcx . ensure_done () . coroutine_by_move_body_def_id (item_def_id) ; } }) ; if tcx . features () . rustc_attrs () { tcx . sess . time ("dumping_rustc_attr_data" , | | { outlives :: dump :: inferred_outlives (tcx) ; variance :: dump :: variances (tcx) ; collect :: dump :: opaque_hidden_types (tcx) ; collect :: dump :: predicates_and_item_bounds (tcx) ; collect :: dump :: def_parents (tcx) ; collect :: dump :: vtables (tcx) ; }) ; } tcx . ensure_ok () . check_unused_traits (()) ; }
/* FP:lib.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_FN_0033
/* FP:lib.rs-0066 */ # [doc = " Lower a [`hir::Ty`] to a [`Ty`]."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " This function is **quasi-deprecated**. It can cause ICEs if called inside of a body"] # [doc = " (of a function or constant) and especially if it contains inferred types (`_`)."] # [doc = ""] # [doc = " It's used in rustdoc and Clippy."] # [doc = ""] # [doc = " </div>"] pub fn lower_ty < 'tcx > (tcx : TyCtxt < 'tcx > , hir_ty : & hir :: Ty < 'tcx >) -> Ty < 'tcx > { let env_def_id = tcx . hir_get_parent_item (hir_ty . hir_id) ; collect :: ItemCtxt :: new (tcx , env_def_id . def_id) . lowerer () . lower_ty_maybe_return_type_notation (hir_ty) }
/* FP:lib.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_lib_FN_0034
/* FP:lib.rs-0068 */ # [doc = " This is for rustdoc."] pub fn lower_const_arg_for_rustdoc < 'tcx > (tcx : TyCtxt < 'tcx > , hir_ct : & hir :: ConstArg < 'tcx > , feed : FeedConstTy < '_ , 'tcx > ,) -> Const < 'tcx > { let env_def_id = tcx . hir_get_parent_item (hir_ct . hir_id) ; collect :: ItemCtxt :: new (tcx , env_def_id . def_id) . lowerer () . lower_const_arg (hir_ct , feed) }