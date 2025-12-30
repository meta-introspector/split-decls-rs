// Generated macro for find_region_in_predicates (function)
macro_rules! Depcrate_check_compare_impl_itemfind_region_in_predicates {
() => {
// Module: crate::check::compare_impl_item
// Provides: {"find_region_in_predicates"}
// Dependencies: {}
fn find_region_in_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , early_bound_region : ty :: Region < 'tcx > ,) -> Option < Span > { for (pred , span) in tcx . explicit_predicates_of (def_id) . instantiate_identity (tcx) { if pred . visit_with (& mut FindRegion (early_bound_region)) . is_break () { return Some (span) ; } } struct FindRegion < 'tcx > (ty :: Region < 'tcx >) ; impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for FindRegion < 'tcx > { type Result = ControlFlow < () > ; fn visit_region (& mut self , r : ty :: Region < 'tcx >) -> Self :: Result { if r == self . 0 { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } } None }
};
}
