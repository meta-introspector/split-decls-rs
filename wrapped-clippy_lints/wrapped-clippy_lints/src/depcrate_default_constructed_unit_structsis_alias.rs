// Generated macro for is_alias (function)
macro_rules! Depcrate_default_constructed_unit_structsis_alias {
() => {
// Module: crate::default_constructed_unit_structs
// Provides: {"is_alias"}
// Dependencies: {}
fn is_alias (ty : hir :: Ty < '_ >) -> bool { if let hir :: TyKind :: Path (ref qpath) = ty . kind { is_ty_alias (qpath) } else { false } }
};
}
