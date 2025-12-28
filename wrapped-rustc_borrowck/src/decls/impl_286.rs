macro_rules! deps {
    () => {
        ConstraintDirection!();
        VarianceExtractor!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < 'tcx > VarianceExtractor < '_ , 'tcx > { fn record_variance (& mut self , region : ty :: Region < 'tcx > , variance : ty :: Variance) { if region . is_bound () { return ; } if region . is_erased () { return ; } let direction = match variance { ty :: Covariant => ConstraintDirection :: Forward , ty :: Contravariant => ConstraintDirection :: Backward , ty :: Invariant => ConstraintDirection :: Bidirectional , ty :: Bivariant => { return ; } } ; let region = self . universal_regions . to_region_vid (region) ; self . directions . entry (region) . and_modify (| entry | { if entry != & direction { * entry = ConstraintDirection :: Bidirectional ; } }) . or_insert (direction) ; } }
    };
}

impl_286!();