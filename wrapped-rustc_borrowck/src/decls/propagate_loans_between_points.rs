macro_rules! deps {
    () => {
        ConstraintDirection!();
        LocalizedOutlivesConstraintSet!();
        UniversalRegions!();
        LocalizedOutlivesConstraint!();
    };
}

macro_rules! propagate_loans_between_points {
    () => {
        deps!();
        # [doc = " Propagate loans within a region between two points in the CFG, if that region is live at both"] # [doc = " the source and target points."] fn propagate_loans_between_points (current_point : PointIndex , next_point : PointIndex , live_regions : & SparseBitMatrix < PointIndex , RegionVid > , live_region_variances : & BTreeMap < RegionVid , ConstraintDirection > , universal_regions : & UniversalRegions < '_ > , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { for region in universal_regions . universal_regions_iter () { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; } let Some (next_live_regions) = live_regions . row (next_point) else { return ; } ; for region in next_live_regions . iter () { if let Some (& direction) = live_region_variances . get (& region) { add_liveness_constraint (region , current_point , next_point , direction , localized_outlives_constraints ,) ; } else { let fallback = ConstraintDirection :: Bidirectional ; add_liveness_constraint (region , current_point , next_point , fallback , localized_outlives_constraints ,) ; } } }
    };
}

propagate_loans_between_points!()