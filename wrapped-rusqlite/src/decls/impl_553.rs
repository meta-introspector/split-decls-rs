macro_rules! deps {
    () => {
        IndexConstraint!();
        IndexConstraintIter!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < 'a > Iterator for IndexConstraintIter < 'a > { type Item = IndexConstraint < 'a > ; # [inline] fn next (& mut self) -> Option < IndexConstraint < 'a > > { self . iter . next () . map (IndexConstraint) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_553!()