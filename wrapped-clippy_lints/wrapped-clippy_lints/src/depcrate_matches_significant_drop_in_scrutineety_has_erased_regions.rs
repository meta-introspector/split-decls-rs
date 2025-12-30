// Generated macro for ty_has_erased_regions (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineety_has_erased_regions {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"ty_has_erased_regions"}
// Dependencies: {}
fn ty_has_erased_regions (ty : Ty < '_ >) -> bool { struct V ; impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for V { type Result = ControlFlow < () > ; fn visit_region (& mut self , region : Region < 'tcx >) -> Self :: Result { if region . is_erased () { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } } ty . visit_with (& mut V) . is_break () }
};
}
