macro_rules! deps {
    () => {
        LocalizedOutlivesConstraint!();
        LocalizedOutlivesConstraintSet!();
        ConstraintDirection!();
    };
}

macro_rules! add_liveness_constraint {
    () => {
        deps!();
        # [doc = " Adds `LocalizedOutlivesConstraint`s between two connected points, according to the given edge"] # [doc = " direction."] fn add_liveness_constraint (region : RegionVid , current_point : PointIndex , next_point : PointIndex , direction : ConstraintDirection , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { match direction { ConstraintDirection :: Forward => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; } ConstraintDirection :: Backward => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : next_point , target : region , to : current_point , }) ; } ConstraintDirection :: Bidirectional => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : next_point , target : region , to : current_point , }) ; } } }
    };
}

add_liveness_constraint!();