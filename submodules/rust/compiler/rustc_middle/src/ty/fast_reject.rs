mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{pub use rustc_type_ir :: fast_reject :: * ;}
mkuse!{use super :: TyCtxt ;}
mkitem!{pub type DeepRejectCtxt < 'tcx , const INSTANTIATE_LHS_WITH_INFER : bool , const INSTANTIATE_RHS_WITH_INFER : bool , > = rustc_type_ir :: fast_reject :: DeepRejectCtxt < TyCtxt < 'tcx > , INSTANTIATE_LHS_WITH_INFER , INSTANTIATE_RHS_WITH_INFER , > ;}
mkitem!{pub type SimplifiedType = rustc_type_ir :: fast_reject :: SimplifiedType < DefId > ;}