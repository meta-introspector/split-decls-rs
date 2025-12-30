// Generated macro for is_lint_ref_type (function)
macro_rules! Depcrate_lint_without_lint_passis_lint_ref_type {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"is_lint_ref_type"}
// Dependencies: {}
pub (super) fn is_lint_ref_type (cx : & LateContext < '_ > , ty : & hir :: Ty < '_ >) -> bool { if let TyKind :: Ref (_ , MutTy { ty : inner , mutbl : Mutability :: Not , } ,) = ty . kind && let TyKind :: Path (ref path) = inner . kind && let Res :: Def (DefKind :: Struct , def_id) = cx . qpath_res (path , inner . hir_id) { internal_paths :: LINT . matches (cx , def_id) } else { false } }
};
}
