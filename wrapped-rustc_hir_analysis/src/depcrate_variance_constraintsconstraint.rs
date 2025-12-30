// Generated macro for Constraint (struct)
macro_rules! Depcrate_variance_constraintsConstraint {
() => {
// Module: crate::variance::constraints
// Provides: {"Constraint"}
// Dependencies: {}
# [doc = " Declares that the variable `decl_id` appears in a location with"] # [doc = " variance `variance`."] # [derive (Copy , Clone)] pub (crate) struct Constraint < 'a > { pub inferred : InferredIndex , pub variance : & 'a VarianceTerm < 'a > , }
};
}
