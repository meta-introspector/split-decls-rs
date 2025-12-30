// Generated macro for impl_626 (impl)
macro_rules! Depcrate_traits_utilimpl_626 {
() => {
// Module: crate::traits::util
// Provides: {"impl_626"}
// Dependencies: {}
# [doc = " For [`Obligation`], a sub-obligation is combined with the current obligation's"] # [doc = " param-env and cause code."] impl < 'tcx > Elaboratable < TyCtxt < 'tcx > > for PredicateObligation < 'tcx > { fn predicate (& self) -> ty :: Predicate < 'tcx > { self . predicate } fn child (& self , clause : ty :: Clause < 'tcx >) -> Self { Obligation { cause : self . cause . clone () , param_env : self . param_env , recursion_depth : 0 , predicate : clause . as_predicate () , } } fn child_with_derived_cause (& self , clause : ty :: Clause < 'tcx > , span : Span , parent_trait_pred : ty :: PolyTraitPredicate < 'tcx > , index : usize ,) -> Self { let cause = self . cause . clone () . derived_cause (parent_trait_pred , | derived | { ObligationCauseCode :: ImplDerived (Box :: new (traits :: ImplDerivedCause { derived , impl_or_alias_def_id : parent_trait_pred . def_id () , impl_def_predicate_index : Some (index) , span , })) }) ; Obligation { cause , param_env : self . param_env , recursion_depth : 0 , predicate : clause . as_predicate () , } } }
};
}
