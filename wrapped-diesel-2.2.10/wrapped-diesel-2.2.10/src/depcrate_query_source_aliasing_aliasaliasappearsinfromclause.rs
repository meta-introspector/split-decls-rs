// Generated macro for AliasAppearsInFromClause (trait)
macro_rules! Depcrate_query_source_aliasing_aliasAliasAppearsInFromClause {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"AliasAppearsInFromClause"}
// Dependencies: {}
# [doc = " This trait is used to allow external crates to implement"] # [doc = " `AppearsInFromClause<QS> for Alias<S>`"] # [doc = ""] # [doc = " at the table level without running in conflicting impl issues"] # [doc = ""] # [doc = " Implementing this at the table level (in the crate that defines the tables)"] # [doc = " does not result in conflicting impl issues because the table is the struct"] # [doc = " we're implementing on."] pub trait AliasAppearsInFromClause < S , QS > { # [doc = " Will be passed on to the `impl AppearsInFromClause<QS>`"] type Count ; }
};
}
