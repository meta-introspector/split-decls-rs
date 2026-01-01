/* FP:common_traits.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_USE_0001
/* FP:common_traits.rs-0002 */ use crate :: rustc_complete :: lang_items :: LangItem ;
/* FP:common_traits.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_USE_0002
/* FP:common_traits.rs-0004 */ use crate :: rustc_infer :: infer :: TyCtxtInferExt ;
/* FP:common_traits.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_USE_0003
/* FP:common_traits.rs-0006 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:common_traits.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_USE_0004
/* FP:common_traits.rs-0008 */ use crate :: rustc_complete :: ty :: { self , Ty , TyCtxt } ;
/* FP:common_traits.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_USE_0005
/* FP:common_traits.rs-0010 */ use crate :: rustc_complete :: DUMMY_SP ;
/* FP:common_traits.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_USE_0006
/* FP:common_traits.rs-0012 */ use crate :: rustc_trait_selection :: traits ;
/* FP:common_traits.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0007
/* FP:common_traits.rs-0014 */ fn is_copy_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Copy) }
/* FP:common_traits.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0008
/* FP:common_traits.rs-0016 */ fn is_use_cloned_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: UseCloned) }
/* FP:common_traits.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0009
/* FP:common_traits.rs-0018 */ fn is_sized_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Sized) }
/* FP:common_traits.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0010
/* FP:common_traits.rs-0020 */ fn is_freeze_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Freeze) }
/* FP:common_traits.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0011
/* FP:common_traits.rs-0022 */ fn is_unpin_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Unpin) }
/* FP:common_traits.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0012
/* FP:common_traits.rs-0024 */ fn is_async_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: AsyncDrop) }
/* FP:common_traits.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0013
/* FP:common_traits.rs-0026 */ fn is_item_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > , item : LangItem ,) -> bool { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (query . typing_env) ; let trait_def_id = tcx . require_lang_item (item , DUMMY_SP) ; traits :: type_known_to_meet_bound_modulo_regions (& infcx , param_env , query . value , trait_def_id) }
/* FP:common_traits.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_common_traits_FN_0014
/* FP:common_traits.rs-0028 */ pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { is_copy_raw , is_use_cloned_raw , is_sized_raw , is_freeze_raw , is_unpin_raw , is_async_drop_raw , .. * providers } ; }