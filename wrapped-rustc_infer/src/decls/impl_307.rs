macro_rules! deps {
    () => {
        PredicateSet!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < 'tcx > Extend < ty :: Predicate < 'tcx > > for PredicateSet < 'tcx > { fn extend < I : IntoIterator < Item = ty :: Predicate < 'tcx > > > (& mut self , iter : I) { for pred in iter { self . insert (pred) ; } } fn extend_one (& mut self , pred : ty :: Predicate < 'tcx >) { self . insert (pred) ; } fn extend_reserve (& mut self , additional : usize) { Extend :: < ty :: Predicate < 'tcx > > :: extend_reserve (& mut self . set , additional) ; } }
    };
}

impl_307!()