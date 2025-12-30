// Generated macro for other_634 (other)
macro_rules! Depcrate_expression_unstableother_634 {
() => {
// Module: crate::expression::unstable
// Provides: {"other_634"}
// Dependencies: {}
# [doc = " Trait alias to represent an expression that isn't aggregate by default."] # [doc = ""] # [doc = " This alias represents a type which is not aggregate if there is no group by"] # [doc = " clause. More specifically, it represents for types which implement"] # [doc = " [`ValidGrouping<()>`] where `IsAggregate` is [`is_aggregate::No`] or"] # [doc = " [`is_aggregate::Yes`]."] # [doc = ""] # [doc = " While this trait is a useful stand-in for common cases, `T: NonAggregate`"] # [doc = " cannot always be used when `T: ValidGrouping<(), IsAggregate = No>` or"] # [doc = " `T: ValidGrouping<(), IsAggregate = Never>` could be. For that reason,"] # [doc = " unless you need to abstract over both columns and literals, you should"] # [doc = " prefer to use [`ValidGrouping<()>`] in your bounds instead."] # [doc = ""] # [doc = " [`ValidGrouping<()>`]: ValidGrouping"] pub trait NonAggregate = ValidGrouping < () > where < Self as ValidGrouping < () > > :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > ;
};
}
