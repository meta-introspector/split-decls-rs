// Generated macro for MixedAggregates (trait)
macro_rules! Depcrate_expressionMixedAggregates {
() => {
// Module: crate::expression
// Provides: {"MixedAggregates"}
// Dependencies: {}
# [doc = " Can two `IsAggregate` types appear in the same expression?"] # [doc = ""] # [doc = " You should never implement this trait. It will eventually become a trait"] # [doc = " alias."] # [doc = ""] # [doc = " [`is_aggregate::Yes`] and [`is_aggregate::No`] can only appear with"] # [doc = " themselves or [`is_aggregate::Never`]. [`is_aggregate::Never`] can appear"] # [doc = " with anything."] # [doc = ""] pub trait MixedAggregates < Other > { # [doc = " What is the resulting `IsAggregate` type?"] type Output ; }
};
}
