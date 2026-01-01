/* FP:check_inline.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0001
/* FP:check_inline.rs-0002 */ use crate :: rustc_complete :: attrs :: InlineAttr ;
/* FP:check_inline.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0002
/* FP:check_inline.rs-0004 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:check_inline.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0003
/* FP:check_inline.rs-0006 */ use crate :: rustc_complete :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;
/* FP:check_inline.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0004
/* FP:check_inline.rs-0008 */ use crate :: rustc_complete :: mir :: { Body , TerminatorKind } ;
/* FP:check_inline.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0005
/* FP:check_inline.rs-0010 */ use crate :: rustc_complete :: ty ;
/* FP:check_inline.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0006
/* FP:check_inline.rs-0012 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:check_inline.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0007
/* FP:check_inline.rs-0014 */ use crate :: rustc_complete :: sym ;
/* FP:check_inline.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_USE_0008
/* FP:check_inline.rs-0016 */ use crate :: pass_manager :: MirLint ;
/* FP:check_inline.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_STRUCT_0009
/* FP:check_inline.rs-0018 */ pub (super) struct CheckForceInline ;
/* FP:check_inline.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_IMPL_0010
/* FP:check_inline.rs-0020 */ impl < 'tcx > MirLint < 'tcx > for CheckForceInline { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { let def_id = body . source . def_id () ; if ! tcx . hir_body_owner_kind (def_id) . is_fn_or_closure () || ! def_id . is_local () { return ; } let InlineAttr :: Force { attr_span , .. } = tcx . codegen_fn_attrs (def_id) . inline else { return ; } ; if let Err (reason) = is_inline_valid_on_fn (tcx , def_id) . and_then (| _ | is_inline_valid_on_body (tcx , body)) { tcx . dcx () . emit_err (crate :: errors :: InvalidForceInline { attr_span , callee_span : tcx . def_span (def_id) , callee : tcx . def_path_str (def_id) , reason , }) ; } } }
/* FP:check_inline.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_FN_0011
/* FP:check_inline.rs-0022 */ pub (super) fn is_inline_valid_on_fn < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId ,) -> Result < () , & 'static str > { let codegen_attrs = tcx . codegen_fn_attrs (def_id) ; if tcx . has_attr (def_id , sym :: rustc_no_mir_inline) { return Err ("#[rustc_no_mir_inline]") ; } let ty = tcx . type_of (def_id) ; if match ty . instantiate_identity () . kind () { ty :: FnDef (..) => tcx . fn_sig (def_id) . instantiate_identity () . c_variadic () , ty :: Closure (_ , args) => args . as_closure () . sig () . c_variadic () , _ => false , } { return Err ("C variadic") ; } if codegen_attrs . flags . contains (CodegenFnAttrFlags :: COLD) { return Err ("cold") ; } if tcx . has_attr (def_id , sym :: rustc_intrinsic) { return Err ("callee is an intrinsic") ; } Ok (()) }
/* FP:check_inline.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_inline_FN_0012
/* FP:check_inline.rs-0024 */ pub (super) fn is_inline_valid_on_body < 'tcx > (_ : TyCtxt < 'tcx > , body : & Body < 'tcx > ,) -> Result < () , & 'static str > { if body . basic_blocks . iter () . any (| bb | matches ! (bb . terminator () . kind , TerminatorKind :: TailCall { .. })) { return Err ("can't inline functions with tail calls") ; } Ok (()) }