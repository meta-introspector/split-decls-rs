// Generated macro for localize_terminator_constraint (function)
macro_rules! Depcrate_polonius_typeck_constraintslocalize_terminator_constraint {
() => {
// Module: crate::polonius::typeck_constraints
// Provides: {"localize_terminator_constraint"}
// Dependencies: {}
# [doc = " For a given outlives constraint arising from a MIR terminator, localize the constraint with the"] # [doc = " needed CFG `from`-`to` inter-block nodes."] fn localize_terminator_constraint < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , terminator : & Terminator < 'tcx > , liveness : & LivenessValues , outlives_constraint : & OutlivesConstraint < 'tcx > , current_point : PointIndex , universal_regions : & UniversalRegions < 'tcx > ,) -> LocalizedOutlivesConstraint { match & terminator . kind { TerminatorKind :: Call { destination , target : Some (target) , .. } => { let destination_ty = destination . ty (& body . local_decls , tcx) ; let successor_location = Location { block : * target , statement_index : 0 } ; let successor_point = liveness . point_from_location (successor_location) ; compute_constraint_direction (tcx , outlives_constraint , & destination_ty , current_point , successor_point , universal_regions ,) } _ => { LocalizedOutlivesConstraint { source : outlives_constraint . sup , from : current_point , target : outlives_constraint . sub , to : current_point , } } } }
};
}
