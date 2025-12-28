macro_rules! deps {
    () => {
        PoloniusContext!();
        BorrowSet!();
        PoloniusDiagnosticsContext!();
        RegionInferenceContext!();
        PoloniusLivenessContext!();
        LocalizedOutlivesConstraintSet!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl PoloniusContext { # [doc = " Unlike NLLs, in polonius we traverse the cfg to look for regions live across an edge, so we"] # [doc = " need to transpose the \"points where each region is live\" matrix to a \"live regions per point\""] # [doc = " matrix."] pub (crate) fn create_from_liveness (liveness_context : PoloniusLivenessContext , num_regions : usize , points_per_live_region : & SparseIntervalMatrix < RegionVid , PointIndex > ,) -> PoloniusContext { let mut live_regions_per_point = SparseBitMatrix :: new (num_regions) ; for region in points_per_live_region . rows () { for point in points_per_live_region . row (region) . unwrap () . iter () { live_regions_per_point . insert (point , region) ; } } PoloniusContext { live_regions : live_regions_per_point , liveness_context } } # [doc = " Computes live loans using the set of loans model for `-Zpolonius=next`."] # [doc = ""] # [doc = " First, creates a constraint graph combining regions and CFG points, by:"] # [doc = " - converting NLL typeck constraints to be localized"] # [doc = " - encoding liveness constraints"] # [doc = ""] # [doc = " Then, this graph is traversed, reachability is recorded as loan liveness, to be used by the"] # [doc = " loan scope and active loans computations."] # [doc = ""] # [doc = " The constraint data will be used to compute errors and diagnostics."] pub (crate) fn compute_loan_liveness < 'tcx > (self , tcx : TyCtxt < 'tcx > , regioncx : & mut RegionInferenceContext < 'tcx > , body : & Body < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> PoloniusDiagnosticsContext { let PoloniusLivenessContext { live_region_variances , boring_nll_locals } = self . liveness_context ; let mut localized_outlives_constraints = LocalizedOutlivesConstraintSet :: default () ; convert_typeck_constraints (tcx , body , regioncx . liveness_constraints () , regioncx . outlives_constraints () , regioncx . universal_regions () , & mut localized_outlives_constraints ,) ; create_liveness_constraints (body , regioncx . liveness_constraints () , & self . live_regions , & live_region_variances , regioncx . universal_regions () , & mut localized_outlives_constraints ,) ; let live_loans = compute_loan_liveness (regioncx . liveness_constraints () , regioncx . outlives_constraints () , borrow_set , & localized_outlives_constraints ,) ; regioncx . record_live_loans (live_loans) ; PoloniusDiagnosticsContext { localized_outlives_constraints , boring_nll_locals } } }
    };
}

impl_304!();