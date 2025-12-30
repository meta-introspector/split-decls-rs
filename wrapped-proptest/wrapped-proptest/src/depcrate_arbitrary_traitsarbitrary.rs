// Generated macro for arbitrary (function)
macro_rules! Depcrate_arbitrary_traitsarbitrary {
() => {
// Module: crate::arbitrary::traits
// Provides: {"arbitrary"}
// Dependencies: {}
# [doc = " Generates a [`Strategy`] producing [`Arbitrary`] values of `A`."] # [doc = " Works better with type inference than [`any::<A>()`]."] # [doc = ""] # [doc = " With this version, you shouldn't need to specify any of the (many) type"] # [doc = " parameters explicitly. This can have a positive effect on type inference."] # [doc = " However, if you want specify `A`, you should use [`any::<A>()`] instead."] # [doc = ""] # [doc = " For clarity, it is often a good idea to specify the type generated, and"] # [doc = " so using [`any::<A>()`] can be a good idea."] # [doc = ""] # [doc = " If you want to customize how the strategy is generated, use"] # [doc = " [`arbitrary_with(args)`] where `args` is of type"] # [doc = " `<A as Arbitrary>::Parameters`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " The function can be used as:"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate proptest;"] # [doc = " use proptest::arbitrary::{arbitrary, StrategyFor};"] # [doc = ""] # [doc = " fn gen_vec_usize() -> StrategyFor<Vec<usize>> {"] # [doc = "     arbitrary()"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " [`arbitrary_with(args)`]: fn.arbitrary_with.html"] # [doc = " [`any::<A>()`]: fn.any.html"] # [doc = " [`Arbitrary`]: trait.Arbitrary.html"] # [doc = " [`Strategy`]: ../strategy/trait.Strategy.html"] # [must_use = "strategies do nothing unless used"] pub fn arbitrary < A , S > () -> S where S : Strategy < Value = A > , A : Arbitrary < Strategy = S > , { A :: arbitrary () }
};
}
