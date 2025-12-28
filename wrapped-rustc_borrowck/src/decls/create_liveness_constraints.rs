macro_rules! deps {
    () => {
        ConstraintDirection!();
        UniversalRegions!();
        LocalizedOutlivesConstraintSet!();
        LivenessValues!();
    };
}

macro_rules! create_liveness_constraints {
    () => {
        deps!();
        # [doc = " Propagate loans throughout the CFG: for each statement in the MIR, create localized outlives"] # [doc = " constraints for loans that are propagated to the next statements."] pub (super) fn create_liveness_constraints < 'tcx > (body : & Body < 'tcx > , liveness : & LivenessValues , live_regions : & SparseBitMatrix < PointIndex , RegionVid > , live_region_variances : & BTreeMap < RegionVid , ConstraintDirection > , universal_regions : & UniversalRegions < 'tcx > , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { for (block , bb) in body . basic_blocks . iter_enumerated () { let statement_count = bb . statements . len () ; for statement_index in 0 ..= statement_count { let current_location = Location { block , statement_index } ; let current_point = liveness . point_from_location (current_location) ; if statement_index < statement_count { let next_location = Location { block , statement_index : statement_index + 1 } ; let next_point = liveness . point_from_location (next_location) ; propagate_loans_between_points (current_point , next_point , live_regions , live_region_variances , universal_regions , localized_outlives_constraints ,) ; } else { for successor_block in bb . terminator () . successors () { let next_location = Location { block : successor_block , statement_index : 0 } ; let next_point = liveness . point_from_location (next_location) ; propagate_loans_between_points (current_point , next_point , live_regions , live_region_variances , universal_regions , localized_outlives_constraints ,) ; } } } } }
    };
}

create_liveness_constraints!()