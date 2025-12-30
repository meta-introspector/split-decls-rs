// Generated macro for add_liveness_constraint (function)
macro_rules! Depcrate_polonius_liveness_constraintsadd_liveness_constraint {
() => {
// Module: crate::polonius::liveness_constraints
// Provides: {"add_liveness_constraint"}
// Dependencies: {}
# [doc = " Adds `LocalizedOutlivesConstraint`s between two connected points, according to the given edge"] # [doc = " direction."] fn add_liveness_constraint (region : RegionVid , current_point : PointIndex , next_point : PointIndex , direction : ConstraintDirection , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { match direction { ConstraintDirection :: Forward => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; } ConstraintDirection :: Backward => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : next_point , target : region , to : current_point , }) ; } ConstraintDirection :: Bidirectional => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : next_point , target : region , to : current_point , }) ; } } }
};
}
