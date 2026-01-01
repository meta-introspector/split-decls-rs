/* FP:dump.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_variance_dump_USE_0001
/* FP:dump.rs-0002 */ use std :: fmt :: Write ;
/* FP:dump.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_variance_dump_USE_0002
/* FP:dump.rs-0004 */ use crate :: rustc_complete :: def_id :: { CRATE_DEF_ID , LocalDefId } ;
/* FP:dump.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_variance_dump_USE_0003
/* FP:dump.rs-0006 */ use crate :: rustc_complete :: ty :: { GenericArgs , TyCtxt } ;
/* FP:dump.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_variance_dump_USE_0004
/* FP:dump.rs-0008 */ use crate :: rustc_complete :: sym ;
/* FP:dump.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_variance_dump_FN_0005
/* FP:dump.rs-0010 */ fn format_variances (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> String { let variances = tcx . variances_of (def_id) ; let generics = GenericArgs :: identity_for_item (tcx , def_id) ; let mut ret = String :: with_capacity (2 + 7 * variances . len ()) ; ret . push ('[') ; for (arg , variance) in generics . iter () . zip (variances . iter ()) { write ! (ret , "{arg}: {variance:?}, ") . unwrap () ; } if ! variances . is_empty () { ret . pop () ; ret . pop () ; } ret . push (']') ; ret }
/* FP:dump.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_variance_dump_FN_0006
/* FP:dump.rs-0012 */ pub (crate) fn variances (tcx : TyCtxt < '_ >) { let crate_items = tcx . hir_crate_items (()) ; if tcx . has_attr (CRATE_DEF_ID , sym :: rustc_variance_of_opaques) { for id in crate_items . opaques () { tcx . dcx () . emit_err (crate :: errors :: VariancesOf { span : tcx . def_span (id) , variances : format_variances (tcx , id) , }) ; } } for id in crate_items . free_items () { if ! tcx . has_attr (id . owner_id , sym :: rustc_variance) { continue ; } tcx . dcx () . emit_err (crate :: errors :: VariancesOf { span : tcx . def_span (id . owner_id) , variances : format_variances (tcx , id . owner_id . def_id) , }) ; } }