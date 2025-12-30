// Generated macro for impl_56 (impl)
macro_rules! Depcrate_lifetimeimpl_56 {
() => {
// Module: crate::lifetime
// Provides: {"impl_56"}
// Dependencies: {}
impl VisitMut for AddLifetimeToImplTrait { fn visit_type_impl_trait_mut (& mut self , ty : & mut TypeImplTrait) { let span = ty . impl_token . span ; let lifetime = parse_quote_spanned ! (span => 'async_trait) ; ty . bounds . insert (0 , lifetime) ; if let Some (punct) = ty . bounds . pairs_mut () . next () . unwrap () . punct_mut () { punct . span = span ; } visit_mut :: visit_type_impl_trait_mut (self , ty) ; } fn visit_type_reference_mut (& mut self , ty : & mut TypeReference) { parenthesize_impl_trait (& mut ty . elem , ty . and_token . span) ; visit_mut :: visit_type_reference_mut (self , ty) ; } fn visit_type_ptr_mut (& mut self , ty : & mut TypePtr) { parenthesize_impl_trait (& mut ty . elem , ty . star_token . span) ; visit_mut :: visit_type_ptr_mut (self , ty) ; } fn visit_type_bare_fn_mut (& mut self , ty : & mut TypeBareFn) { if let ReturnType :: Type (arrow , return_type) = & mut ty . output { parenthesize_impl_trait (return_type , arrow . spans [0]) ; } visit_mut :: visit_type_bare_fn_mut (self , ty) ; } fn visit_expr_mut (& mut self , _e : & mut Expr) { } }
};
}
