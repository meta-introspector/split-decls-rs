// Generated macro for impl_4948 (impl)
macro_rules! Depcrate_matches_significant_drop_in_scrutineeimpl_4948 {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"impl_4948"}
// Dependencies: {}
impl < 'a , 'tcx > SigDropChecker < 'a , 'tcx > { fn new (cx : & 'a LateContext < 'tcx >) -> SigDropChecker < 'a , 'tcx > { SigDropChecker { seen_types : FxHashSet :: default () , cx , } } fn is_sig_drop_expr (& mut self , ex : & 'tcx Expr < '_ >) -> bool { ! ex . is_syntactic_place_expr () && self . has_sig_drop_attr (self . cx . typeck_results () . expr_ty (ex)) } fn has_sig_drop_attr (& mut self , ty : Ty < 'tcx >) -> bool { self . seen_types . clear () ; self . has_sig_drop_attr_impl (ty) } fn has_sig_drop_attr_impl (& mut self , ty : Ty < 'tcx >) -> bool { if let Some (adt) = ty . ty_adt_def () && get_builtin_attr (self . cx . sess () , self . cx . tcx . get_all_attrs (adt . did ()) , sym :: has_significant_drop ,) . count () > 0 { return true ; } if ! self . seen_types . insert (ty) { return false ; } match ty . kind () { rustc_middle :: ty :: Adt (adt , args) => { adt . all_fields () . map (| field | field . ty (self . cx . tcx , args)) . any (| ty | self . has_sig_drop_attr_impl (ty)) || (args . iter () . all (| arg | ! matches ! (arg . kind () , GenericArgKind :: Lifetime (_))) && args . iter () . filter_map (| arg | match arg . kind () { GenericArgKind :: Type (ty) => Some (ty) , _ => None , }) . any (| ty | self . has_sig_drop_attr_impl (ty))) } , rustc_middle :: ty :: Tuple (tys) => tys . iter () . any (| ty | self . has_sig_drop_attr_impl (ty)) , rustc_middle :: ty :: Array (ty , _) | rustc_middle :: ty :: Slice (ty) => self . has_sig_drop_attr_impl (* ty) , _ => false , } } }
};
}
