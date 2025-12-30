// Generated macro for are_equal (function)
macro_rules! Depcrate_operators_op_refare_equal {
() => {
// Module: crate::operators::op_ref
// Provides: {"are_equal"}
// Dependencies: {}
fn are_equal (cx : & LateContext < '_ > , middle_ty : Ty < '_ > , hir_ty : & rustc_hir :: Ty < '_ >) -> bool { if let ty :: Adt (adt_def , _) = middle_ty . kind () && let Some (local_did) = adt_def . did () . as_local () && let item = cx . tcx . hir_expect_item (local_did) && let middle_ty_id = item . owner_id . to_def_id () && let TyKind :: Path (QPath :: Resolved (_ , path)) = hir_ty . kind && let Res :: Def (_ , hir_ty_id) = path . res { hir_ty_id == middle_ty_id } else { false } }
};
}
