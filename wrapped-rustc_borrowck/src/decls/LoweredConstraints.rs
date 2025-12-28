macro_rules! deps {
    () => {
        PlaceholderIndices!();
        OutlivesConstraintSet!();
        UniverseInfo!();
        RegionTracker!();
        LivenessValues!();
        RegionDefinition!();
        TypeTest!();
    };
}

macro_rules! LoweredConstraints {
    () => {
        deps!();
        # [doc = " A set of outlives constraints after rewriting to remove"] # [doc = " higher-kinded constraints."] pub (crate) struct LoweredConstraints < 'tcx > { pub (crate) constraint_sccs : Sccs < RegionVid , ConstraintSccIndex > , pub (crate) definitions : Frozen < IndexVec < RegionVid , RegionDefinition < 'tcx > > > , pub (crate) scc_annotations : IndexVec < ConstraintSccIndex , RegionTracker > , pub (crate) outlives_constraints : Frozen < OutlivesConstraintSet < 'tcx > > , pub (crate) type_tests : Vec < TypeTest < 'tcx > > , pub (crate) liveness_constraints : LivenessValues , pub (crate) universe_causes : FxIndexMap < UniverseIndex , UniverseInfo < 'tcx > > , pub (crate) placeholder_indices : PlaceholderIndices , }
    };
}

LoweredConstraints!()