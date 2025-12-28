macro_rules! deps {
    () => {
        TypeChecker!();
    };
}

macro_rules! generate {
    () => {
        deps!();
        # [doc = " Combines liveness analysis with initialization analysis to"] # [doc = " determine which variables are live at which points, both due to"] # [doc = " ordinary uses and drops. Returns a set of (ty, location) pairs"] # [doc = " that indicate which types must be live at which point in the CFG."] # [doc = " This vector is consumed by `constraint_generation`."] # [doc = ""] # [doc = " N.B., this computation requires normalization; therefore, it must be"] # [doc = " performed before"] pub (super) fn generate < 'tcx > (typeck : & mut TypeChecker < '_ , 'tcx > , location_map : & DenseLocationMap , move_data : & MoveData < 'tcx > ,) { debug ! ("liveness::generate") ; let mut free_regions = regions_that_outlive_free_regions (typeck . infcx . num_region_vars () , & typeck . universal_regions , & typeck . constraints . outlives_constraints ,) ; if typeck . tcx () . sess . opts . unstable_opts . polonius . is_next_enabled () { let (_ , boring_locals) = compute_relevant_live_locals (typeck . tcx () , & free_regions , typeck . body) ; typeck . polonius_liveness . as_mut () . unwrap () . boring_nll_locals = boring_locals . into_iter () . collect () ; free_regions = typeck . universal_regions . universal_regions_iter () . collect () ; } let (relevant_live_locals , boring_locals) = compute_relevant_live_locals (typeck . tcx () , & free_regions , typeck . body) ; trace :: trace (typeck , location_map , move_data , relevant_live_locals , boring_locals) ; record_regular_live_regions (typeck . tcx () , & mut typeck . constraints . liveness_constraints , & typeck . universal_regions , & mut typeck . polonius_liveness , typeck . body ,) ; }
    };
}

generate!();