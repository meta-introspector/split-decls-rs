// Generated macro for LifetimeChecker (struct)
macro_rules! Depcrate_lifetimesLifetimeChecker {
() => {
// Module: crate::lifetimes
// Provides: {"LifetimeChecker"}
// Dependencies: {}
struct LifetimeChecker < 'cx , 'tcx , F > { cx : & 'cx LateContext < 'tcx > , map : FxIndexMap < LocalDefId , Vec < Usage > > , where_predicate_depth : usize , bounded_ty_depth : usize , generic_args_depth : usize , lifetime_elision_impossible : bool , phantom : std :: marker :: PhantomData < F > , }
};
}
