// Generated macro for TableNotEqual (trait)
macro_rules! Depcrate_query_sourceTableNotEqual {
() => {
// Module: crate::query_source
// Provides: {"TableNotEqual"}
// Dependencies: {}
# [doc = " Allows Diesel to implement some internal traits for two tables that are distinct."] # [doc = ""] # [doc = " (Notably, a bunch of [`AppearsInFromClause`] for the tables and their aliases.)"] # [doc = ""] # [doc = " This trait is implemented by the [`allow_tables_to_appear_in_same_query!`] macro."] # [doc = ""] # [doc = " Troubleshooting"] # [doc = " ---------------"] # [doc = " If you encounter an error mentioning this trait, it could mean that either:"] # [doc = " - You are attempting to use tables that don't belong to the same database together"] # [doc = "   (no call to [`allow_tables_to_appear_in_same_query!`] was made)"] # [doc = " - You are attempting to use two aliases to the same table in the same query, but they"] # [doc = "   were declared through different calls to [`alias!`](crate::alias)"] # [diagnostic :: on_unimplemented (note = "double check that `{T}` and `{Self}` appear in the same `allow_tables_to_appear_in_same_query!` \ncall if both are tables")] pub trait TableNotEqual < T : Table > : Table { }
};
}
