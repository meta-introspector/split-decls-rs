// Generated macro for impl_1602 (impl)
macro_rules! Depcrate_disallowed_typesimpl_1602 {
() => {
// Module: crate::disallowed_types
// Provides: {"impl_1602"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DisallowedTypes { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if let ItemKind :: Use (path , UseKind :: Single (_)) = & item . kind && let Some (res) = path . res . type_ns { self . check_res_emit (cx , & res , item . span) ; } } fn check_ty (& mut self , cx : & LateContext < 'tcx > , ty : & 'tcx Ty < 'tcx , AmbigArg >) { if let TyKind :: Path (path) = & ty . kind { self . check_res_emit (cx , & cx . qpath_res (path , ty . hir_id) , ty . span) ; } } fn check_poly_trait_ref (& mut self , cx : & LateContext < 'tcx > , poly : & 'tcx PolyTraitRef < 'tcx >) { self . check_res_emit (cx , & poly . trait_ref . path . res , poly . trait_ref . path . span) ; } }
};
}
