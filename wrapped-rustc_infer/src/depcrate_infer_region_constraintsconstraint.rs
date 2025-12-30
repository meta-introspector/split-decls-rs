// Generated macro for Constraint (struct)
macro_rules! Depcrate_infer_region_constraintsConstraint {
() => {
// Module: crate::infer::region_constraints
// Provides: {"Constraint"}
// Dependencies: {}
# [doc = " Represents a constraint that influences the inference process."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub struct Constraint < 'tcx > { pub kind : ConstraintKind , pub sub : Region < 'tcx > , pub sup : Region < 'tcx > , }
};
}
