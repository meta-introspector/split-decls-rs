// Generated macro for impl_752 (impl)
macro_rules! Depcrate_polonius_liveness_constraintsimpl_752 {
() => {
// Module: crate::polonius::liveness_constraints
// Provides: {"impl_752"}
// Dependencies: {}
impl < 'tcx > VarianceExtractor < '_ , 'tcx > { fn record_variance (& mut self , region : ty :: Region < 'tcx > , variance : ty :: Variance) { if region . is_bound () { return ; } if region . is_erased () { return ; } let direction = match variance { ty :: Covariant => ConstraintDirection :: Forward , ty :: Contravariant => ConstraintDirection :: Backward , ty :: Invariant => ConstraintDirection :: Bidirectional , ty :: Bivariant => { return ; } } ; let region = self . universal_regions . to_region_vid (region) ; self . directions . entry (region) . and_modify (| entry | { if entry != & direction { * entry = ConstraintDirection :: Bidirectional ; } }) . or_insert (direction) ; } }
};
}
