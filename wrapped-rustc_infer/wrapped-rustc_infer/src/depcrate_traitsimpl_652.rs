// Generated macro for impl_652 (impl)
macro_rules! Depcrate_traitsimpl_652 {
() => {
// Module: crate::traits
// Provides: {"impl_652"}
// Dependencies: {}
impl < 'tcx > PredicateObligation < 'tcx > { # [doc = " Flips the polarity of the inner predicate."] # [doc = ""] # [doc = " Given `T: Trait` predicate it returns `T: !Trait` and given `T: !Trait` returns `T: Trait`."] pub fn flip_polarity (& self , tcx : TyCtxt < 'tcx >) -> Option < PredicateObligation < 'tcx > > { Some (PredicateObligation { cause : self . cause . clone () , param_env : self . param_env , predicate : self . predicate . flip_polarity (tcx) ? , recursion_depth : self . recursion_depth , }) } }
};
}
