macro_rules! deps {
    () => {
        ConstraintGraphDirection!();
        Successors!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , 'tcx , D : ConstraintGraphDirection > Iterator for Successors < 'a , 'tcx , D > { type Item = RegionVid ; fn next (& mut self) -> Option < Self :: Item > { match self { Successors :: FromStatic (edges) => { edges . next () } Successors :: FromGraph (edges) => { edges . next () . map (| constraint | D :: end_region (constraint . sup , constraint . sub)) } } } }
    };
}

impl_33!();