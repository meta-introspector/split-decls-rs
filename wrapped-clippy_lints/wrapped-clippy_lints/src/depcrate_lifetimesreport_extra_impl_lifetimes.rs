// Generated macro for report_extra_impl_lifetimes (function)
macro_rules! Depcrate_lifetimesreport_extra_impl_lifetimes {
() => {
// Module: crate::lifetimes
// Provides: {"report_extra_impl_lifetimes"}
// Dependencies: {}
fn report_extra_impl_lifetimes < 'tcx > (cx : & LateContext < 'tcx > , impl_ : & 'tcx Impl < '_ >) { let mut checker = LifetimeChecker :: < middle_nested_filter :: All > :: new (cx , impl_ . generics) ; walk_generics (& mut checker , impl_ . generics) ; if let Some (of_trait) = impl_ . of_trait { walk_trait_ref (& mut checker , & of_trait . trait_ref) ; } walk_unambig_ty (& mut checker , impl_ . self_ty) ; for & item in impl_ . items { walk_impl_item_ref (& mut checker , item) ; } for (& def_id , usages) in & checker . map { if usages . iter () . all (| usage | usage . in_where_predicate && ! usage . in_bounded_ty && ! usage . in_generics_arg) { span_lint (cx , EXTRA_UNUSED_LIFETIMES , cx . tcx . def_span (def_id) , "this lifetime isn't used in the impl" ,) ; } } report_elidable_impl_lifetimes (cx , impl_ , & checker . map) ; }
};
}
