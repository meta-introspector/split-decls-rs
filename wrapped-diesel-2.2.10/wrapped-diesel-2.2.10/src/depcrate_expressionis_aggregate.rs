// Generated macro for is_aggregate (module)
macro_rules! Depcrate_expressionis_aggregate {
() => {
// Module: crate::expression
// Provides: {"is_aggregate"}
// Dependencies: {}
# [allow (missing_debug_implementations , missing_copy_implementations)] # [doc = " Possible values for `ValidGrouping::IsAggregate`"] pub mod is_aggregate { use super :: MixedAggregates ; # [doc = " Yes, this expression is aggregate for the given group by clause."] pub struct Yes ; # [doc = " No, this expression is not aggregate with the given group by clause,"] # [doc = " but it might be aggregate with a different group by clause."] pub struct No ; # [doc = " This expression is never aggregate, and can appear with any other"] # [doc = " expression, regardless of whether it is aggregate."] # [doc = ""] # [doc = " Examples of this are literals. `1` does not care about aggregation."] # [doc = " `foo + 1` is always valid, regardless of whether `foo` appears in the"] # [doc = " group by clause or not."] pub struct Never ; impl MixedAggregates < Yes > for Yes { type Output = Yes ; } impl MixedAggregates < Never > for Yes { type Output = Yes ; } impl MixedAggregates < No > for No { type Output = No ; } impl MixedAggregates < Never > for No { type Output = No ; } impl < T > MixedAggregates < T > for Never { type Output = T ; } }
};
}
