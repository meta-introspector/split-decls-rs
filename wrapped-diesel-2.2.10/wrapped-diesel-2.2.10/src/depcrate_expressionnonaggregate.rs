// Generated macro for NonAggregate (trait)
macro_rules! Depcrate_expressionNonAggregate {
() => {
// Module: crate::expression
// Provides: {"NonAggregate"}
// Dependencies: {}
# [doc = " Trait alias to represent an expression that isn't aggregate by default."] # [doc = ""] # [doc = " This trait should never be implemented directly. It is replaced with a"] # [doc = " trait alias when the `unstable` feature is enabled."] # [doc = ""] # [doc = " This alias represents a type which is not aggregate if there is no group by"] # [doc = " clause. More specifically, it represents for types which implement"] # [doc = " [`ValidGrouping<()>`] where `IsAggregate` is [`is_aggregate::No`] or"] # [doc = " [`is_aggregate::Yes`]."] # [doc = ""] # [doc = " While this trait is a useful stand-in for common cases, `T: NonAggregate`"] # [doc = " cannot always be used when `T: ValidGrouping<(), IsAggregate = No>` or"] # [doc = " `T: ValidGrouping<(), IsAggregate = Never>` could be. For that reason,"] # [doc = " unless you need to abstract over both columns and literals, you should"] # [doc = " prefer to use [`ValidGrouping<()>`] in your bounds instead."] # [doc = ""] # [doc = " [`ValidGrouping<()>`]: ValidGrouping"] # [cfg (not (feature = "unstable"))] pub trait NonAggregate : ValidGrouping < () > { }
};
}
