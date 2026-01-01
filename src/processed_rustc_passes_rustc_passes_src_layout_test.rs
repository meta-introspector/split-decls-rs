/* FP:layout_test.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0001
/* FP:layout_test.rs-0002 */ use crate :: rustc_abi :: { HasDataLayout , TargetDataLayout } ;
/* FP:layout_test.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0002
/* FP:layout_test.rs-0004 */ use crate :: rustc_complete :: Attribute ;
/* FP:layout_test.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0003
/* FP:layout_test.rs-0006 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:layout_test.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0004
/* FP:layout_test.rs-0008 */ use crate :: rustc_complete :: def_id :: LocalDefId ;
/* FP:layout_test.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0005
/* FP:layout_test.rs-0010 */ use crate :: rustc_complete :: span_bug ;
/* FP:layout_test.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0006
/* FP:layout_test.rs-0012 */ use crate :: rustc_complete :: ty :: layout :: { HasTyCtxt , HasTypingEnv , LayoutError , LayoutOfHelpers } ;
/* FP:layout_test.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0007
/* FP:layout_test.rs-0014 */ use crate :: rustc_complete :: ty :: { self , Ty , TyCtxt } ;
/* FP:layout_test.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0008
/* FP:layout_test.rs-0016 */ use crate :: rustc_complete :: source_map :: Spanned ;
/* FP:layout_test.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0009
/* FP:layout_test.rs-0018 */ use crate :: rustc_complete :: { Span , sym } ;
/* FP:layout_test.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0010
/* FP:layout_test.rs-0020 */ use crate :: rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;
/* FP:layout_test.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0011
/* FP:layout_test.rs-0022 */ use crate :: rustc_trait_selection :: infer :: TyCtxtInferExt ;
/* FP:layout_test.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0012
/* FP:layout_test.rs-0024 */ use crate :: rustc_trait_selection :: traits ;
/* FP:layout_test.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_USE_0013
/* FP:layout_test.rs-0026 */ use crate :: errors :: { LayoutAbi , LayoutAlign , LayoutHomogeneousAggregate , LayoutInvalidAttribute , LayoutOf , LayoutSize , UnrecognizedArgument , } ;
/* FP:layout_test.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_FN_0014
/* FP:layout_test.rs-0028 */ pub fn test_layout (tcx : TyCtxt < '_ >) { if ! tcx . features () . rustc_attrs () { return ; } for id in tcx . hir_crate_items (()) . definitions () { for attr in tcx . get_attrs (id , sym :: rustc_layout) { match tcx . def_kind (id) { DefKind :: TyAlias | DefKind :: Enum | DefKind :: Struct | DefKind :: Union => { dump_layout_of (tcx , id , attr) ; } _ => { tcx . dcx () . emit_err (LayoutInvalidAttribute { span : tcx . def_span (id) }) ; } } } } }
/* FP:layout_test.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_FN_0015
/* FP:layout_test.rs-0030 */ pub fn ensure_wf < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , def_id : LocalDefId , span : Span ,) -> bool { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let ocx = traits :: ObligationCtxt :: new_with_diagnostics (& infcx) ; let pred = ty :: ClauseKind :: WellFormed (ty . into ()) ; let obligation = traits :: Obligation :: new (tcx , traits :: ObligationCause :: new (span , def_id , traits :: ObligationCauseCode :: WellFormed (Some (traits :: WellFormedLoc :: Ty (def_id))) ,) , param_env , pred ,) ; ocx . register_obligation (obligation) ; let errors = ocx . select_all_or_error () ; if ! errors . is_empty () { infcx . err_ctxt () . report_fulfillment_errors (errors) ; false } else { true } }
/* FP:layout_test.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_FN_0016
/* FP:layout_test.rs-0032 */ fn dump_layout_of (tcx : TyCtxt < '_ > , item_def_id : LocalDefId , attr : & Attribute) { let typing_env = ty :: TypingEnv :: post_analysis (tcx , item_def_id) ; let ty = tcx . type_of (item_def_id) . instantiate_identity () ; let span = tcx . def_span (item_def_id . to_def_id ()) ; if ! ensure_wf (tcx , typing_env , ty , item_def_id , span) { return ; } match tcx . layout_of (typing_env . as_query_input (ty)) { Ok (ty_layout) => { let meta_items = attr . meta_item_list () . unwrap_or_default () ; for meta_item in meta_items { match meta_item . name () { Some (sym :: abi) => { tcx . dcx () . emit_err (LayoutAbi { span , abi : format ! ("{:?}" , ty_layout . backend_repr) , }) ; } Some (sym :: align) => { tcx . dcx () . emit_err (LayoutAlign { span , align : format ! ("{:?}" , ty_layout . align) , }) ; } Some (sym :: size) => { tcx . dcx () . emit_err (LayoutSize { span , size : format ! ("{:?}" , ty_layout . size) }) ; } Some (sym :: homogeneous_aggregate) => { tcx . dcx () . emit_err (LayoutHomogeneousAggregate { span , homogeneous_aggregate : format ! ("{:?}" , ty_layout . homogeneous_aggregate (& UnwrapLayoutCx { tcx , typing_env })) , }) ; } Some (sym :: debug) => { let normalized_ty = tcx . normalize_erasing_regions (typing_env , ty) ; let ty_layout = format ! ("{:#?}" , * ty_layout) ; tcx . dcx () . emit_err (LayoutOf { span , normalized_ty , ty_layout }) ; } _ => { tcx . dcx () . emit_err (UnrecognizedArgument { span : meta_item . span () }) ; } } } } Err (layout_error) => { tcx . dcx () . emit_err (Spanned { node : layout_error . into_diagnostic () , span }) ; } } }
/* FP:layout_test.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_STRUCT_0017
/* FP:layout_test.rs-0034 */ struct UnwrapLayoutCx < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , }
/* FP:layout_test.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_IMPL_0018
/* FP:layout_test.rs-0036 */ impl < 'tcx > LayoutOfHelpers < 'tcx > for UnwrapLayoutCx < 'tcx > { fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { span_bug ! (span , "`#[rustc_layout(..)]` test resulted in `layout_of({ty}) = Err({err})`" ,) ; } }
/* FP:layout_test.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_IMPL_0019
/* FP:layout_test.rs-0038 */ impl < 'tcx > HasTyCtxt < 'tcx > for UnwrapLayoutCx < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
/* FP:layout_test.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_IMPL_0020
/* FP:layout_test.rs-0040 */ impl < 'tcx > HasTypingEnv < 'tcx > for UnwrapLayoutCx < 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env } }
/* FP:layout_test.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_layout_test_IMPL_0021
/* FP:layout_test.rs-0042 */ impl < 'tcx > HasDataLayout for UnwrapLayoutCx < 'tcx > { fn data_layout (& self) -> & TargetDataLayout { self . tcx . data_layout () } }