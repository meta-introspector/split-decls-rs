macro_rules! deps {
    () => {
        RegionDefinition!();
    };
}

macro_rules! SccAnnotations {
    () => {
        deps!();
        # [doc = " A Visitor for SCC annotation construction."] pub (crate) struct SccAnnotations < 'd , 'tcx , A : scc :: Annotation > { pub (crate) scc_to_annotation : IndexVec < ConstraintSccIndex , A > , definitions : & 'd IndexVec < RegionVid , RegionDefinition < 'tcx > > , }
    };
}

SccAnnotations!()