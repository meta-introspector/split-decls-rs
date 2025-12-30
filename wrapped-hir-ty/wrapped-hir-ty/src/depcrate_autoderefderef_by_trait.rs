// Generated macro for deref_by_trait (function)
macro_rules! Depcrate_autoderefderef_by_trait {
() => {
// Module: crate::autoderef
// Provides: {"deref_by_trait"}
// Dependencies: {}
pub (crate) fn deref_by_trait (table @ & mut InferenceTable { db , .. } : & mut InferenceTable < '_ > , ty : Ty , use_receiver_trait : bool ,) -> Option < Ty > { let _p = tracing :: info_span ! ("deref_by_trait") . entered () ; if table . resolve_ty_shallow (& ty) . inference_var (Interner) . is_some () { return None ; } let trait_id = | | { # [expect (clippy :: overly_complex_bool_expr)] if use_receiver_trait && false && let Some (receiver) = LangItem :: Receiver . resolve_trait (db , table . trait_env . krate) { return Some (receiver) ; } LangItem :: Deref . resolve_trait (db , table . trait_env . krate) } ; let trait_id = trait_id () ? ; let target = trait_id . trait_items (db) . associated_type_by_name (& Name :: new_symbol_root (sym :: Target)) ? ; let projection = { let b = TyBuilder :: subst_for_def (db , trait_id , None) ; if b . remaining () != 1 { return None ; } let deref_subst = b . push (ty) . build () ; TyBuilder :: assoc_type_projection (db , target , Some (deref_subst)) . build () } ; let trait_ref = projection . trait_ref (db) ; let implements_goal : Goal = trait_ref . cast (Interner) ; table . try_obligation (implements_goal . clone ()) ? ; table . register_obligation (implements_goal) ; let result = table . normalize_projection_ty (projection) ; Some (table . resolve_ty_shallow (& result)) }
};
}
