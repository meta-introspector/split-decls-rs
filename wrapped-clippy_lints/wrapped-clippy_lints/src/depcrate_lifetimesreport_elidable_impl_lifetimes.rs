// Generated macro for report_elidable_impl_lifetimes (function)
macro_rules! Depcrate_lifetimesreport_elidable_impl_lifetimes {
() => {
// Module: crate::lifetimes
// Provides: {"report_elidable_impl_lifetimes"}
// Dependencies: {}
fn report_elidable_impl_lifetimes < 'tcx > (cx : & LateContext < 'tcx > , impl_ : & 'tcx Impl < '_ > , map : & FxIndexMap < LocalDefId , Vec < Usage > > ,) { let (elidable_lts , usages) : (Vec < _ > , Vec < _ >) = map . iter () . filter_map (| (def_id , usages) | { if let [Usage { lifetime , in_where_predicate : false , lifetime_elision_impossible : false , .. } ,] = usages . as_slice () { Some ((def_id , lifetime)) } else { None } }) . unzip () ; if elidable_lts . is_empty () { return ; } report_elidable_lifetimes (cx , impl_ . generics , & elidable_lts , & usages , true) ; }
};
}
