// Generated macro for ParamsFor (type)
macro_rules! Depcrate_arbitrary_traitsParamsFor {
() => {
// Module: crate::arbitrary::traits
// Provides: {"ParamsFor"}
// Dependencies: {}
# [doc = " `ParamsFor` allows you to mention the type of [`Parameters`] for the input"] # [doc = " type `A` without directly using associated types or without resorting to"] # [doc = " existential types. This way, if implementation of [`Arbitrary`] changes,"] # [doc = " your tests should not break."] # [doc = ""] # [doc = " [`Parameters`]: trait.Arbitrary.html#associatedtype.Parameters"] # [doc = " [`Arbitrary`]: trait.Arbitrary.html"] # [doc = " [`Strategy`]: ../strategy/trait.Strategy.html"] pub type ParamsFor < A > = < A as Arbitrary > :: Parameters ;
};
}
