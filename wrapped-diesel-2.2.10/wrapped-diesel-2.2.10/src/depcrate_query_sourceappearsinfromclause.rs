// Generated macro for AppearsInFromClause (trait)
macro_rules! Depcrate_query_sourceAppearsInFromClause {
() => {
// Module: crate::query_source
// Provides: {"AppearsInFromClause"}
// Dependencies: {}
# [doc = " Determines how many times `Self` appears in `QS`"] # [doc = ""] # [doc = " This trait is primarily used to determine whether or not a column is"] # [doc = " selectable from a given from clause. A column can be selected if its table"] # [doc = " appears in the from clause *exactly once*."] # [doc = ""] # [doc = " We do not allow the same table to appear in a query multiple times in any"] # [doc = " context where referencing that table would be ambiguous (depending on the"] # [doc = " context and backend being used, this may or may not be something that would"] # [doc = " otherwise result in a runtime error)."] # [diagnostic :: on_unimplemented (note = "double check that `{QS}` and `{Self}` appear in the same `allow_tables_to_appear_in_same_query!` \ncall if both are tables" , note = "double check that any two aliases to the same table in `{QS}` and `{Self}` appear in the same `alias!` call")] pub trait AppearsInFromClause < QS > { # [doc = " How many times does `Self` appear in `QS`?"] type Count ; }
};
}
