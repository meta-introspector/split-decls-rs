// Generated macro for impl_295 (impl)
macro_rules! Depcrate_infer_projectionimpl_295 {
() => {
// Module: crate::infer::projection
// Provides: {"impl_295"}
// Dependencies: {}
impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Instead of normalizing an associated type projection,"] # [doc = " this function generates an inference variable and registers"] # [doc = " an obligation that this inference variable must be the result"] # [doc = " of the given projection. This allows us to proceed with projections"] # [doc = " while they cannot be resolved yet due to missing information or"] # [doc = " simply due to the lack of access to the trait resolution machinery."] pub fn projection_term_to_infer (& self , param_env : ty :: ParamEnv < 'tcx > , alias_term : ty :: AliasTerm < 'tcx > , cause : ObligationCause < 'tcx > , recursion_depth : usize , obligations : & mut PredicateObligations < 'tcx > ,) -> Term < 'tcx > { debug_assert ! (! self . next_trait_solver ()) ; let span = self . tcx . def_span (alias_term . def_id) ; let infer_var = if alias_term . kind (self . tcx) . is_type () { self . next_ty_var (span) . into () } else { self . next_const_var (span) . into () } ; let projection = ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (ty :: ProjectionPredicate { projection_term : alias_term , term : infer_var , })) ; let obligation = Obligation :: with_depth (self . tcx , cause , recursion_depth , param_env , projection) ; obligations . push (obligation) ; infer_var } }
};
}
