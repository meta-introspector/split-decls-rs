macro_rules! deps {
    () => {
        EdgesFromGraph!();
        OutlivesConstraint!();
        ConstraintGraphDirection!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a , 'tcx , D : ConstraintGraphDirection > Iterator for EdgesFromGraph < 'a , 'tcx , D > { type Item = & 'a OutlivesConstraint < 'tcx > ; fn next (& mut self) -> Option < Self :: Item > { if let Some (p) = self . pointer { self . pointer = self . graph . next_constraints [p] ; Some (& self . constraints [p]) } else { None } } }
    };
}

impl_27!();