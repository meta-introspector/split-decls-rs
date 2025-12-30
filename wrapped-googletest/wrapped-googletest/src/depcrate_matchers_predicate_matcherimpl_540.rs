// Generated macro for impl_540 (impl)
macro_rules! Depcrate_matchers_predicate_matcherimpl_540 {
() => {
// Module: crate::matchers::predicate_matcher
// Provides: {"impl_540"}
// Dependencies: {}
impl < T , S > PredicateDescription for T where T : Fn () -> S , S : Into < String > , { fn to_description (& self) -> Description { self () . into () . into () } }
};
}
