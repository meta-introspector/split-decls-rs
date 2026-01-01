/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [feature (array_windows)] # [feature (file_buffered)] # [feature (if_let_guard)] # [feature (impl_trait_in_assoc_type)] # [feature (once_cell_get_mut)] use crate :: rustc_complete :: lang_items :: LangItem ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0002
/* FP:lib.rs-0004 */ use crate :: rustc_complete :: query :: TyCtxtAt ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0003
/* FP:lib.rs-0006 */ use crate :: rustc_complete :: ty :: adjustment :: CustomCoerceUnsized ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0004
/* FP:lib.rs-0008 */ use crate :: rustc_complete :: ty :: { self , Ty } ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0006
/* FP:lib.rs-0012 */ use crate :: rustc_complete :: { bug , traits } ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_USE_0007
/* FP:lib.rs-0014 */ use crate :: rustc_complete :: ErrorGuaranteed ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_MOD_0009
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_MOD_0010
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_MOD_0011
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_MOD_0012
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_MACRO_0013
/* FP:lib.rs-0026 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_FN_0014
/* FP:lib.rs-0028 */ fn custom_coerce_unsize_info < 'tcx > (tcx : TyCtxtAt < 'tcx > , source_ty : Ty < 'tcx > , target_ty : Ty < 'tcx > ,) -> Result < CustomCoerceUnsized , ErrorGuaranteed > { let trait_ref = ty :: TraitRef :: new (tcx . tcx , tcx . require_lang_item (LangItem :: CoerceUnsized , tcx . span) , [source_ty , target_ty] ,) ; match tcx . codegen_select_candidate (ty :: TypingEnv :: fully_monomorphized () . as_query_input (trait_ref)) { Ok (traits :: ImplSource :: UserDefined (traits :: ImplSourceUserDefinedData { impl_def_id , .. })) => Ok (tcx . coerce_unsized_info (impl_def_id) ? . custom_kind . unwrap ()) , impl_source => { bug ! ("invalid `CoerceUnsized` from {source_ty} to {target_ty}: impl_source: {:?}" , impl_source) ; } } }
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_lib_FN_0015
/* FP:lib.rs-0030 */ pub fn provide (providers : & mut Providers) { partitioning :: provide (providers) ; mono_checks :: provide (providers) ; }