// Generated macro for ValidGrouping (trait)
macro_rules! Depcrate_expressionValidGrouping {
() => {
// Module: crate::expression
// Provides: {"ValidGrouping"}
// Dependencies: {}
# [doc = " Is this expression valid for a given group by clause?"] # [doc = ""] # [doc = " Implementations of this trait must ensure that aggregate expressions are"] # [doc = " not mixed with non-aggregate expressions."] # [doc = ""] # [doc = " For generic types, you can determine if your sub-expressions can appear"] # [doc = " together using the [`MixedAggregates`] trait."] # [doc = ""] # [doc = " `GroupByClause` will be a tuple containing the set of expressions appearing"] # [doc = " in the `GROUP BY` portion of the query. If there is no `GROUP BY`, it will"] # [doc = " be `()`."] # [doc = ""] # [doc = " This trait can be [derived]"] # [doc = ""] # [doc = " [derived]: derive@ValidGrouping"] pub trait ValidGrouping < GroupByClause > { # [doc = " Is this expression aggregate?"] # [doc = ""] # [doc = " This type should always be one of the structs in the [`is_aggregate`]"] # [doc = " module. See the documentation of those structs for more details."] # [doc = ""] type IsAggregate ; }
};
}
