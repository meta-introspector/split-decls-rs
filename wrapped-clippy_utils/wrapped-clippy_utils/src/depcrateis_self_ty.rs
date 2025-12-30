// Generated macro for is_self_ty (function)
macro_rules! Depcrateis_self_ty {
() => {
// Module: crate
// Provides: {"is_self_ty"}
// Dependencies: {}
pub fn is_self_ty (slf : & hir :: Ty < '_ >) -> bool { if let TyKind :: Path (QPath :: Resolved (None , path)) = slf . kind && let Res :: SelfTyParam { .. } | Res :: SelfTyAlias { .. } = path . res { return true ; } false }
};
}
