// Generated macro for PredicateView (struct)
macro_rules! Depcrate_settingsPredicateView {
() => {
// Module: crate::settings
// Provides: {"PredicateView"}
// Dependencies: {}
# [doc = " A reference to just the boolean predicates of a settings object."] # [doc = ""] # [doc = " The settings objects themselves are generated and appear in the `isa/*/settings.rs` modules."] # [doc = " Each settings object provides a `predicate_view()` method that makes it possible to query"] # [doc = " ISA predicates by number."] # [derive (Clone , Copy , Hash)] pub struct PredicateView < 'a > (& 'a [u8]) ;
};
}
