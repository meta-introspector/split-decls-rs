// Generated macro for impl_10582 (impl)
macro_rules! Depcrate_types_type_complexityimpl_10582 {
() => {
// Module: crate::types::type_complexity
// Provides: {"impl_10582"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for TypeComplexityVisitor { fn visit_infer (& mut self , inf_id : hir :: HirId , _inf_span : Span , _kind : InferKind < 'tcx >) -> Self :: Result { self . score += 1 ; self . visit_id (inf_id) ; } fn visit_ty (& mut self , ty : & 'tcx hir :: Ty < '_ , AmbigArg >) { let (add_score , sub_nest) = match ty . kind { TyKind :: Ptr (..) | TyKind :: Ref (..) => (1 , 0) , TyKind :: Path (..) | TyKind :: Slice (..) | TyKind :: Tup (..) | TyKind :: Array (..) => (10 * self . nest , 1) , TyKind :: FnPtr (fn_ptr) if fn_ptr . abi == ExternAbi :: Rust => (50 * self . nest , 1) , TyKind :: TraitObject (param_bounds , _) => { let has_lifetime_parameters = param_bounds . iter () . any (| bound | { bound . bound_generic_params . iter () . any (| param | matches ! (param . kind , GenericParamKind :: Lifetime { .. })) }) ; if has_lifetime_parameters { (50 * self . nest , 1) } else { (20 * self . nest , 0) } } , _ => (0 , 0) , } ; self . score += add_score ; self . nest += sub_nest ; walk_ty (self , ty) ; self . nest -= sub_nest ; } }
};
}
