macro_rules! deps {
    () => {
        RegionValues!();
        NormalConstraintGraph!();
        RegionTracker!();
        TypeTest!();
        LivenessValues!();
        OutlivesConstraintSet!();
        UniversalRegionRelations!();
        UniverseInfo!();
        ConstraintSccs!();
        RegionDefinition!();
    };
}

macro_rules! RegionInferenceContext {
    () => {
        deps!();
        pub struct RegionInferenceContext < 'tcx > { # [doc = " Contains the definition for every region variable. Region"] # [doc = " variables are identified by their index (`RegionVid`). The"] # [doc = " definition contains information about where the region came"] # [doc = " from as well as its final inferred value."] pub (crate) definitions : Frozen < IndexVec < RegionVid , RegionDefinition < 'tcx > > > , # [doc = " The liveness constraints added to each region. For most"] # [doc = " regions, these start out empty and steadily grow, though for"] # [doc = " each universally quantified region R they start out containing"] # [doc = " the entire CFG and `end(R)`."] liveness_constraints : LivenessValues , # [doc = " The outlives constraints computed by the type-check."] constraints : Frozen < OutlivesConstraintSet < 'tcx > > , # [doc = " The constraint-set, but in graph form, making it easy to traverse"] # [doc = " the constraints adjacent to a particular region. Used to construct"] # [doc = " the SCC (see `constraint_sccs`) and for error reporting."] constraint_graph : Frozen < NormalConstraintGraph > , # [doc = " The SCC computed from `constraints` and the constraint"] # [doc = " graph. We have an edge from SCC A to SCC B if `A: B`. Used to"] # [doc = " compute the values of each region."] constraint_sccs : ConstraintSccs , scc_annotations : IndexVec < ConstraintSccIndex , RegionTracker > , # [doc = " Map universe indexes to information on why we created it."] universe_causes : FxIndexMap < ty :: UniverseIndex , UniverseInfo < 'tcx > > , # [doc = " The final inferred values of the region variables; we compute"] # [doc = " one value per SCC. To get the value for any given *region*,"] # [doc = " you first find which scc it is a part of."] scc_values : RegionValues < ConstraintSccIndex > , # [doc = " Type constraints that we check after solving."] type_tests : Vec < TypeTest < 'tcx > > , # [doc = " Information about how the universally quantified regions in"] # [doc = " scope on this function relate to one another."] universal_region_relations : Frozen < UniversalRegionRelations < 'tcx > > , }
    };
}

RegionInferenceContext!();