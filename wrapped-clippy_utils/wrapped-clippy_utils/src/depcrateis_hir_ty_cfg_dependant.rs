// Generated macro for is_hir_ty_cfg_dependant (function)
macro_rules! Depcrateis_hir_ty_cfg_dependant {
() => {
// Module: crate
// Provides: {"is_hir_ty_cfg_dependant"}
// Dependencies: {}
pub fn is_hir_ty_cfg_dependant (cx : & LateContext < '_ > , ty : & hir :: Ty < '_ >) -> bool { if let TyKind :: Path (QPath :: Resolved (_ , path)) = ty . kind && let Res :: Def (_ , def_id) = path . res { return cx . tcx . has_attr (def_id , sym :: cfg) || cx . tcx . has_attr (def_id , sym :: cfg_attr) ; } false }
};
}
