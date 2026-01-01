/* FP:structural_match.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_USE_0001
/* FP:structural_match.rs-0002 */ use crate :: rustc_complete :: lang_items :: LangItem ;
/* FP:structural_match.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_USE_0002
/* FP:structural_match.rs-0004 */ use crate :: rustc_infer :: infer :: TyCtxtInferExt ;
/* FP:structural_match.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_USE_0003
/* FP:structural_match.rs-0006 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:structural_match.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_USE_0004
/* FP:structural_match.rs-0008 */ use crate :: rustc_complete :: ty :: { self , Ty , TyCtxt , TypingMode } ;
/* FP:structural_match.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_USE_0005
/* FP:structural_match.rs-0010 */ use crate :: rustc_trait_selection :: traits :: { ObligationCause , ObligationCtxt } ;
/* FP:structural_match.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_FN_0006
/* FP:structural_match.rs-0012 */ # [doc = " This method returns true if and only if `adt_ty` itself has been marked as"] # [doc = " eligible for structural-match: namely, if it implements"] # [doc = " `StructuralPartialEq` (which is injected by `#[derive(PartialEq)]`)."] # [doc = ""] # [doc = " Note that this does *not* recursively check if the substructure of `adt_ty`"] # [doc = " implements the trait."] fn has_structural_eq_impl < 'tcx > (tcx : TyCtxt < 'tcx > , adt_ty : Ty < 'tcx >) -> bool { let infcx = & tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let cause = ObligationCause :: dummy () ; let ocx = ObligationCtxt :: new (infcx) ; let structural_peq_def_id = infcx . tcx . require_lang_item (LangItem :: StructuralPeq , cause . span) ; ocx . register_bound (cause . clone () , ty :: ParamEnv :: empty () , adt_ty , structural_peq_def_id) ; ocx . select_all_or_error () . is_empty () }
/* FP:structural_match.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_structural_match_FN_0007
/* FP:structural_match.rs-0014 */ pub (crate) fn provide (providers : & mut Providers) { providers . has_structural_eq_impl = has_structural_eq_impl ; }