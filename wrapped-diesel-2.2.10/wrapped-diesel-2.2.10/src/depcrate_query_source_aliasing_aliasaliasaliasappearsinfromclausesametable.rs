// Generated macro for AliasAliasAppearsInFromClauseSameTable (trait)
macro_rules! Depcrate_query_source_aliasing_aliasAliasAliasAppearsInFromClauseSameTable {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"AliasAliasAppearsInFromClauseSameTable"}
// Dependencies: {}
# [doc = " This trait is used to allow external crates to implement"] # [doc = " `AppearsInFromClause<Alias<S2>> for Alias<S1>`"] # [doc = ""] # [doc = " at the alias level without running in conflicting impl issues"] # [doc = ""] # [doc = " Implementing this at the alias level (in the crate that defines the aliases)"] # [doc = " does not result in conflicting impl issues because the aliases are specified as both first"] # [doc = " argument of the trait and struct we're implementing on."] # [doc = ""] # [doc = " The corresponding implementation of `AliasAliasAppearsInFromClause` for implementors of this"] # [doc = " trait is done at the table level, to avoid conflict with the implementation for distinct tables"] # [doc = " below."] pub trait AliasAliasAppearsInFromClauseSameTable < S2 , T > { # [doc = " Will be passed on to the `impl AppearsInFromClause<QS>`"] type Count ; }
};
}
