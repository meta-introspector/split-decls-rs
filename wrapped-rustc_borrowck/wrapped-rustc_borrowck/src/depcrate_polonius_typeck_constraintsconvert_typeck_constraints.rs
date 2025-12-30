// Generated macro for convert_typeck_constraints (function)
macro_rules! Depcrate_polonius_typeck_constraintsconvert_typeck_constraints {
() => {
// Module: crate::polonius::typeck_constraints
// Provides: {"convert_typeck_constraints"}
// Dependencies: {}
# [doc = " Propagate loans throughout the subset graph at a given point (with some subtleties around the"] # [doc = " location where effects start to be visible)."] pub (super) fn convert_typeck_constraints < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , liveness : & LivenessValues , outlives_constraints : impl Iterator < Item = OutlivesConstraint < 'tcx > > , universal_regions : & UniversalRegions < 'tcx > , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { for outlives_constraint in outlives_constraints { match outlives_constraint . locations { Locations :: All (_) => { continue ; } Locations :: Single (location) => { let point = liveness . point_from_location (location) ; let localized_constraint = if let Some (stmt) = body [location . block] . statements . get (location . statement_index) { localize_statement_constraint (tcx , body , stmt , & outlives_constraint , point , universal_regions ,) } else { assert_eq ! (location . statement_index , body [location . block] . statements . len ()) ; let terminator = body [location . block] . terminator () ; localize_terminator_constraint (tcx , body , terminator , liveness , & outlives_constraint , point , universal_regions ,) } ; localized_outlives_constraints . push (localized_constraint) ; } } } }
};
}
