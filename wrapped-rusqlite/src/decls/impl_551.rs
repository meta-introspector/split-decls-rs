macro_rules! deps {
    () => {
        IndexConstraintUsage!();
        IndexConstraintAndUsageIter!();
        IndexConstraint!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        impl < 'a > Iterator for IndexConstraintAndUsageIter < 'a > { type Item = (IndexConstraint < 'a > , IndexConstraintUsage < 'a >) ; # [inline] fn next (& mut self) -> Option < (IndexConstraint < 'a > , IndexConstraintUsage < 'a >) > { self . iter . next () . map (| raw | (IndexConstraint (raw . 0) , IndexConstraintUsage (raw . 1))) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_551!();