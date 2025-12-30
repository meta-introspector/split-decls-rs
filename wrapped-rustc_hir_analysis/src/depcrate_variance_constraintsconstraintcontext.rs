// Generated macro for ConstraintContext (struct)
macro_rules! Depcrate_variance_constraintsConstraintContext {
() => {
// Module: crate::variance::constraints
// Provides: {"ConstraintContext"}
// Dependencies: {}
pub (crate) struct ConstraintContext < 'a , 'tcx > { pub terms_cx : TermsContext < 'a , 'tcx > , covariant : VarianceTermPtr < 'a > , contravariant : VarianceTermPtr < 'a > , invariant : VarianceTermPtr < 'a > , bivariant : VarianceTermPtr < 'a > , pub constraints : Vec < Constraint < 'a > > , }
};
}
