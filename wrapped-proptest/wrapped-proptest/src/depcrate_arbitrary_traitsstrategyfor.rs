// Generated macro for StrategyFor (type)
macro_rules! Depcrate_arbitrary_traitsStrategyFor {
() => {
// Module: crate::arbitrary::traits
// Provides: {"StrategyFor"}
// Dependencies: {}
# [doc = " `StrategyFor` allows you to mention the type of [`Strategy`] for the input"] # [doc = " type `A` without directly using associated types or without resorting to"] # [doc = " existential types. This way, if implementation of [`Arbitrary`] changes,"] # [doc = " your tests should not break. This can be especially beneficial when the"] # [doc = " type of `Strategy` that you are dealing with is very long in name"] # [doc = " (the case with generics)."] # [doc = ""] # [doc = " [`Arbitrary`]: trait.Arbitrary.html"] # [doc = " [`Strategy`]: ../strategy/trait.Strategy.html"] pub type StrategyFor < A > = < A as Arbitrary > :: Strategy ;
};
}
