// Generated macro for compute_constraint_direction (function)
macro_rules! Depcrate_polonius_typeck_constraintscompute_constraint_direction {
() => {
// Module: crate::polonius::typeck_constraints
// Provides: {"compute_constraint_direction"}
// Dependencies: {}
# [doc = " For a given outlives constraint and CFG edge, returns the localized constraint with the"] # [doc = " appropriate `from`-`to` direction. This is computed according to whether the constraint flows to"] # [doc = " or from a free region in the given `value`, some kind of result for an effectful operation, like"] # [doc = " the LHS of an assignment."] fn compute_constraint_direction < 'tcx > (tcx : TyCtxt < 'tcx > , outlives_constraint : & OutlivesConstraint < 'tcx > , value : & impl TypeVisitable < TyCtxt < 'tcx > > , current_point : PointIndex , successor_point : PointIndex , universal_regions : & UniversalRegions < 'tcx > ,) -> LocalizedOutlivesConstraint { let mut to = current_point ; let mut from = current_point ; tcx . for_each_free_region (value , | region | { let region = universal_regions . to_region_vid (region) ; if region == outlives_constraint . sub { to = successor_point ; } else if region == outlives_constraint . sup { from = successor_point ; } }) ; LocalizedOutlivesConstraint { source : outlives_constraint . sup , from , target : outlives_constraint . sub , to , } }
};
}
