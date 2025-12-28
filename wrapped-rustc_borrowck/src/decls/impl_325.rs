macro_rules! deps {
    () => {
        SccConstraints!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < 'a , 'this , 'tcx > dot :: GraphWalk < 'this > for SccConstraints < 'a , 'tcx > { type Node = ConstraintSccIndex ; type Edge = (ConstraintSccIndex , ConstraintSccIndex) ; fn nodes (& 'this self) -> dot :: Nodes < 'this , ConstraintSccIndex > { let vids : Vec < ConstraintSccIndex > = self . regioncx . constraint_sccs . all_sccs () . collect () ; vids . into () } fn edges (& 'this self) -> dot :: Edges < 'this , (ConstraintSccIndex , ConstraintSccIndex) > { let edges : Vec < _ > = self . regioncx . constraint_sccs . all_sccs () . flat_map (| scc_a | { self . regioncx . constraint_sccs . successors (scc_a) . iter () . map (move | & scc_b | (scc_a , scc_b)) }) . collect () ; edges . into () } fn source (& 'this self , edge : & (ConstraintSccIndex , ConstraintSccIndex)) -> ConstraintSccIndex { edge . 0 } fn target (& 'this self , edge : & (ConstraintSccIndex , ConstraintSccIndex)) -> ConstraintSccIndex { edge . 1 } }
    };
}

impl_325!();