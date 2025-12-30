// Generated macro for AliasAliasAppearsInFromClause (trait)
macro_rules! Depcrate_query_source_aliasing_aliasAliasAliasAppearsInFromClause {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"AliasAliasAppearsInFromClause"}
// Dependencies: {}
# [doc = " This trait is used to allow external crates to implement"] # [doc = " `AppearsInFromClause<Alias<S2>> for Alias<S1>`"] # [doc = ""] # [doc = " at the table level without running in conflicting impl issues"] # [doc = ""] # [doc = " Implementing this at the table level (in the crate that defines the tables)"] # [doc = " does not result in conflicting impl issues because the tables are specified as both first"] # [doc = " argument of the trait and struct we're implementing on."] pub trait AliasAliasAppearsInFromClause < T2 , S1 , S2 > { # [doc = " Will be passed on to the `impl AppearsInFromClause<QS>`"] type Count ; }
};
}
