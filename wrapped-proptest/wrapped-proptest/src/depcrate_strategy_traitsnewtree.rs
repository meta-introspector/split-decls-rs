// Generated macro for NewTree (type)
macro_rules! Depcrate_strategy_traitsNewTree {
() => {
// Module: crate::strategy::traits
// Provides: {"NewTree"}
// Dependencies: {}
# [doc = " A new [`ValueTree`] from a [`Strategy`] when [`Ok`] or otherwise [`Err`]"] # [doc = " when a new value-tree can not be produced for some reason such as"] # [doc = " in the case of filtering with a predicate which always returns false."] # [doc = " You should pass in your strategy as the type parameter."] # [doc = ""] # [doc = " [`Strategy`]: trait.Strategy.html"] # [doc = " [`ValueTree`]: trait.ValueTree.html"] # [doc = " [`Ok`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Ok"] # [doc = " [`Err`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Err"] pub type NewTree < S > = Result < < S as Strategy > :: Tree , Reason > ;
};
}
